use itertools::Itertools;

use crate::Solution;

#[derive(Debug, Clone, Copy)]
struct Lock([u8; 5]);

#[derive(Debug, Clone, Copy)]
struct Key([u8; 5]);

fn fits(lock: &Lock, key: &Key) -> bool {
    lock.0.iter().zip(key.0.iter()).all(|(l, k)| l + k <= 5)
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut locks = vec![];
    let mut keys = vec![];

    for group in input.split("\n\n") {
        if group.starts_with("#") {
            let mut lock = Lock([0; 5]);
            for line in group.lines().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock.0[i] += 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = Key([0; 5]);
            for line in group.lines().rev().skip(1) {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        key.0[i] += 1;
                    }
                }
            }
            keys.push(key);
        }
    }

    let part1 = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| fits(lock, key))
        .count();

    Solution(part1, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day25.txt")) == crate::Solution(3, 0));
    }
}
