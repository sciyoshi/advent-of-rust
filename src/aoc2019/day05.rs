use super::intcode::exec;
use crate::utils::extract_integers;
use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut ops: Vec<_> = extract_integers::<isize>(input).into_iter().collect();

    let mut inp = vec![1isize].into_iter();
    let mut out = vec![];
    let mut ip = 0;

    while let Some(next_ip) = exec(&mut ops, ip, &mut inp, &mut out) {
        ip = next_ip;
    }

    println!("{:?}", out);

    Solution(0, 0)
}
