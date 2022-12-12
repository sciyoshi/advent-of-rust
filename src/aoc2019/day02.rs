use crate::Solution;
use std::iter::empty;

use super::intcode::exec;
use crate::utils::extract_integers;

pub fn solve(input: &str) -> Solution<isize, isize> {
    let ops: Vec<_> = extract_integers::<isize>(input).into_iter().collect();

    let mut ip = 0;

    let mut ops1 = ops.clone();

    ops1[1] = 12;
    ops1[2] = 2;

    let mut inp = empty();
    let mut out = vec![];

    while let Some(next_ip) = exec(&mut ops1, ip, &mut inp, &mut out) {
        ip = next_ip;
    }

    let part1 = ops1[0];

    for i in 0..100 {
        for j in 0..100 {
            let mut ops2 = ops.clone();
            ip = 0;

            ops2[1] = i;
            ops2[2] = j;

            while let Some(next_ip) = exec(&mut ops2, ip, &mut inp, &mut out) {
                ip = next_ip;
            }

            if ops2[0] == 19690720 {
                return Solution(part1, 100 * i + j);
            }
        }
    }

    unreachable!();
}
