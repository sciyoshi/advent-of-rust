use crate::Solution;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution<u32, u32> {
    let mut heights = HashMap::<u32, u32>::new();

    for line in input.lines() {
        let split: Vec<_> = line.split(": ").collect();

        heights.insert(split[0].parse().unwrap(), split[1].parse().unwrap());
    }

    let severity: u32 = heights
        .iter()
        .filter(|&(&pos, &height)| pos % (2 * (height - 1)) == 0)
        .map(|(pos, height)| pos * height)
        .sum();

    let wait: u32 = (0..)
        .filter(|wait| {
            !heights
                .iter()
                .any(|(&pos, &height)| (wait + pos) % (2 * (height - 1)) == 0)
        })
        .next()
        .unwrap();

    Solution(severity, wait)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("0: 3\n1: 2\n4: 4\n6: 4") == crate::Solution(24, 10));
    }
}
