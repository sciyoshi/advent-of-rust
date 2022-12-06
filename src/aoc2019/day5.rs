use std::io::stdin;
use std::iter::empty;

use super::intcode::exec;
use crate::utils::extract_integers;

pub fn solve() {
    let line = stdin().lines().next().unwrap().unwrap();

    let mut ops: Vec<_> = extract_integers::<isize>(&line).into_iter().collect();

    let mut inp = vec![1isize].into_iter();
    let mut out = vec![];
    let mut ip = 0;

    while let Some(next_ip) = exec(&mut ops, ip, &mut inp, &mut out) {
        ip = next_ip;
    }

    println!("{:?}", out);
}
