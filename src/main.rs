use std::{
    env::args,
    io::{self, Write},
    iter::once,
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;
use ndarray::{s, Array2};

use crate::factors::{factors, gcd_many};

mod factors;

#[derive(Clone, Debug)]
struct Tableau<F> {
    inner: Array2<F>,
}

impl Tableau<i32> {
    fn new(n: usize) -> Self {
        Self {
            inner: Array2::from_elem((n, n + 1), 0),
        }
    }

    fn print(&self) {
        let n = self.inner.shape()[0];
        let m = self.inner.shape()[1];
        for i in 0..n {
            for j in 0..m - 1 {
                print!("{:5}", self.inner[(i, j)]);
            }
            let row = self.inner.slice(s![i, ..]);
            let lcm = gcd_many(row.iter().cloned());
            let spacing = " ".repeat(5);
            println!(
                "{}| {:5}{}divisors: {:?}",
                &spacing,
                self.inner[(i, m - 1)],
                &spacing,
                factors(lcm)
            );
        }
        println!();
    }

    fn checked_row_idx(&self, row: usize) -> Result<usize, String> {
        if row > self.inner.shape()[0] - 1 {
            Err("Bad idx".to_string())
        } else {
            Ok(row)
        }
    }

    fn add_row_mult_to_row(
        &mut self,
        src_row: usize,
        dst_row: usize,
        mult: i32,
    ) -> Result<(), String> {
        let src_row = self.checked_row_idx(src_row)?;
        let dst_row = self.checked_row_idx(dst_row)?;
        let src = self.inner.slice(s![src_row, ..]).to_owned();
        let mut dst = self.inner.slice_mut(s![dst_row, ..]);
        for (vs, vd) in src.iter().zip(dst.iter_mut()) {
            *vd += *vs * mult;
        }
        Ok(())
    }

    fn swap_rows(&mut self, r1: usize, r2: usize) -> Result<(), String> {
        let r1 = self.checked_row_idx(r1)?;
        let r2 = self.checked_row_idx(r2)?;
        let row1 = self.inner.slice(s![r1, ..]).to_owned();
        let row2 = self.inner.slice(s![r2, ..]).to_owned();
        self.inner.slice_mut(s![r1, ..]).assign(&row2);
        self.inner.slice_mut(s![r2, ..]).assign(&row1);
        Ok(())
    }

    fn mult_row(&mut self, row: usize, mult: i32) -> Result<(), String> {
        let row = self.checked_row_idx(row)?;
        for v in self.inner.slice_mut(s![row, ..]).iter_mut() {
            *v *= mult
        }
        Ok(())
    }

    fn div_row(&mut self, row: usize, mult: i32) -> Result<(), String> {
        let row = self.checked_row_idx(row)?;
        let gcd = gcd_many(self.inner.slice(s![row, ..]).iter().cloned());
        match factors(gcd).contains(&mult) {
            true => {
                for v in self.inner.slice_mut(s![row, ..]).iter_mut() {
                    *v /= mult
                }
                Ok(())
            }
            false => Err("multiple not in divisors".to_string()),
        }
    }
}

impl FromStr for Tableau<i32> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals: Vec<i32> = vec![];
        for vs in s.split(|c| c == ',' || c == ';') {
            vals.push(vs.parse::<i32>().map_err(|e| e.to_string())?);
        }

        let mut n = 2;
        loop {
            let ts = n * (n + 1);
            if n > 100 {
                panic!("Thats a lot of CSVs");
            }
            if vals.len() == ts {
                break;
            }
            if vals.len() < ts {
                return Err("Incorrect dimensions".to_string());
            }
            n += 1;
        }

        let mut new = Tableau::<i32>::new(n);
        for (vi, v) in vals.into_iter().zip(new.inner.iter_mut()) {
            *v = vi;
        }
        Ok(new)
    }
}

#[derive(Parser, Debug)]
enum Command {
    Set {
        /// Comma separated values, can also use semicolon for own convenience, expected len n*(n+1).
        /// Example input: 1,0,2;0,2,3 (no spaces)
        new_tab: Tableau<i32>,
    },
    Swap {
        row1: usize,
        row2: usize,
    },
    Mult {
        row: usize,
        mult: i32,
    },
    Div {
        row: usize,
        mult: i32,
    },
    /// add mult copies of the src_row to the dst_row
    Add {
        src_row: usize,
        dst_row: usize,
        mult: i32,
    },
    Quit,
}

struct Prog {
    tableau: Tableau<i32>,
    quit: bool,
    cmd: Command,
}

impl Prog {
    fn run_cmd(&mut self) -> Result<(), String> {
        match &self.cmd {
            Command::Quit => {
                self.quit = true;
                Ok(())
            }
            Command::Set { new_tab } => {
                self.tableau = new_tab.clone();
                Ok(())
            }

            Command::Swap { row1, row2 } => self.tableau.swap_rows(*row1, *row2),
            Command::Mult { row, mult } => self.tableau.mult_row(*row, *mult),
            Command::Div { row, mult } => self.tableau.div_row(*row, *mult),
            Command::Add {
                src_row,
                dst_row,
                mult,
            } => self.tableau.add_row_mult_to_row(*src_row, *dst_row, *mult),
        }
    }

    fn try_parse(&mut self, input: &str) -> Result<(), clap::error::Error> {
        let prog_path: PathBuf = args().next().unwrap().into();
        let prog_name = prog_path.file_name().unwrap().to_str().unwrap();
        let n = prog_name.len();
        // whitespace split not ideal?
        if &input[..n] == prog_name {
            println!("\"{prog_name}\" may be ommited");
            Command::try_parse_from(input.split_whitespace())
        } else {
            Command::try_parse_from(once("").chain(input.split_whitespace()))
        }
        .map(|c| self.cmd = c)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut prog = Prog {
        tableau: Tableau::new(2),
        quit: false,
        cmd: Command::Quit,
    };
    print!(
        "Run \"help\" for available commands, and further \"help <COMMAND>\" for command use.\n\n"
    );
    loop {
        print!(" > ");
        _ = stdout.flush();
        input.clear();
        _ = stdin.read_line(&mut input);
        println!();
        match prog.try_parse(input.trim()) {
            Ok(_) => match prog.run_cmd() {
                Ok(_) => {
                    if prog.quit {
                        break;
                    }
                    prog.tableau.print()
                }
                Err(e) => println!("{e}"),
            },
            Err(e) => {
                println!("{}", e.to_string());
            }
        }
    }
}
