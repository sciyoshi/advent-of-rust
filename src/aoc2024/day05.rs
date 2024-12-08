use std::cmp::Ordering;

use itertools::Itertools;

use crate::{Solution, utils::extract_integers};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();

    let mut order = vec![vec![Ordering::Equal; 100]; 100];

    for rule in rules.lines().map(extract_integers::<usize>) {
        order[rule[0]][rule[1]] = Ordering::Less;
        order[rule[1]][rule[0]] = Ordering::Greater;
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for update in updates.lines() {
        let mut pages = extract_integers::<usize>(update);

        if pages
            .iter()
            .tuple_windows()
            .all(|(a, b)| order[*a][*b] == Ordering::Less)
        {
            part1 += pages[pages.len() / 2];
        } else {
            pages.sort_by(|a, b| order[*a][*b]);
            part2 += pages[pages.len() / 2];
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day05.txt")) == crate::Solution(143, 123));
    }
}
