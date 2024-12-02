use std::collections::BTreeSet;

use crate::Solution;

fn priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        item as u32 - 'A' as u32 + 27
    } else {
        item as u32 - 'a' as u32 + 1
    }
}

pub fn solve(input: &str) -> Solution<u32, u32> {
    let part1: u32 = input
        .lines()
        .map(|l| {
            let set1 = l[..l.len() / 2].chars().collect::<BTreeSet<char>>();
            let set2 = l[l.len() / 2..].chars().collect::<BTreeSet<char>>();

            priority(*set1.iter().find(|c| set2.contains(c)).unwrap())
        })
        .sum();

    let part2: u32 = input
        .lines()
        .array_chunks::<3>()
        .map(|l| {
            let set1 = l[0].chars().collect::<BTreeSet<char>>();
            let set2 = l[1].chars().collect::<BTreeSet<char>>();
            let set3 = l[2].chars().collect::<BTreeSet<char>>();

            priority(
                *set1
                    .iter().find(|c| set2.contains(c) && set3.contains(c))
                    .unwrap(),
            )
        })
        .sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day3.txt")) == crate::Solution(157, 70));
    }
}
