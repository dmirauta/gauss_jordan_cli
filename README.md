# Gauss Jordan operations CLI

(Ab)uses a clap parser in a loop to interactively take commands for operations to perform, mutating an integer tableau.

Example input/output:
```
Run "help" for available commands, and further "help <COMMAND>" for command use.

 > help

Usage: gjc <COMMAND>

Commands:
  set   
  swap  
  mult  
  div   
  add   add mult copies of the src_row to the dst_row
  quit  
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

 > help set

Usage: set <NEW_TAB>

Arguments:
  <NEW_TAB>  Comma separated values, can also use semicolon for own convenience, expected len n*(n+1). Example input: 1,0,2;0,2,3 (no spaces)

Options:
  -h, --help  Print help

 > set 1,0,2;0,2,3

    1    0     |     2     divisors: {1}
    0    2     |     3     divisors: {1}

 > mult 0 45

   45    0     |    90     divisors: {45, 9, 1, 15, 3, 5}
    0    2     |     3     divisors: {1}

 > div 0 5

    9    0     |    18     divisors: {9, 3, 1}
    0    2     |     3     divisors: {1}

 >
```
