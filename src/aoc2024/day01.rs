// Generated with ChatGPT 4.

use std::iter::zip;

use counter::Counter;
use itertools::Itertools;

use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    for line in input.lines() {
        let (a, b) = line.split_whitespace().collect_tuple().unwrap();
        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    }

    left.sort();
    right.sort();

    let part1: usize = zip(&left, &right).map(|(&a, &b)| a.abs_diff(b)).sum();

    let counts = right.iter().collect::<Counter<_>>();

    let part2 = left.iter().map(|x| x * counts[x]).sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day01.txt")) == crate::Solution(11, 31));
    }
}
