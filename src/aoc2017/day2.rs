use crate::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution<u32, u32> {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|el| el.parse::<u32>().ok())
                .collect()
        })
        .collect();

    // First checksum: max - min of each line, summed
    let checksum1: u32 = lines
        .iter()
        .map(|els| els.iter().max().unwrap() - els.iter().min().unwrap())
        .sum();

    // Second checksum: check all pairs, and find the first that is evenly divisible
    let checksum2: u32 = lines
        .iter()
        .filter_map(|els| {
            els.iter()
                .tuple_combinations()
                .filter_map(|(a, b)| {
                    if a % b == 0 {
                        Some(a / b)
                    } else if b % a == 0 {
                        Some(b / a)
                    } else {
                        None
                    }
                })
                .next()
        })
        .sum();

    Solution(checksum1, checksum2)
}
