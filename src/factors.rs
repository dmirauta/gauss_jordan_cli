#![allow(dead_code)]

use std::collections::HashSet;

pub fn euclids(a: i32, b: i32) -> i32 {
    if b.abs() > a.abs() {
        return euclids(b, a);
    }
    let r = a % b;
    match r == 0 {
        true => b,
        false => euclids(b, r),
    }
}

pub fn lcm(a: i32, b: i32) -> i32 {
    a * b / euclids(a, b)
}

pub fn gcd_many(itr: impl Iterator<Item = i32>) -> i32 {
    itr.filter(|n| *n > 0)
        .reduce(|acc, n| euclids(acc, n))
        .expect("iterator should be non-empty")
}

pub fn lcm_many(itr: impl Iterator<Item = i32>) -> i32 {
    itr.filter(|n| *n > 0)
        .reduce(|acc, n| lcm(acc, n))
        .expect("iterator should be non-empty")
}

pub fn factors(n: i32) -> HashSet<i32> {
    let n = n.abs();
    let mut fact = HashSet::new();
    let upper = (n as f32).sqrt().floor() as i32;
    for i in 1..=upper {
        if n % i == 0 {
            fact.insert(i);
            fact.insert(n / i);
        }
    }
    fact
}

#[test]
fn _factors() {
    dbg!(euclids(-15, 6));
    dbg!(lcm(-15, 6));
    dbg!(factors(-15));
}
