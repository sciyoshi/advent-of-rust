use std::collections::HashMap;

use itertools::Itertools;

use crate::Solution;
use regex::Regex;

struct Towels<'a> {
    towels: &'a [&'a str],
    cache: HashMap<&'a str, usize>,
}

fn ways<'a>(towels: &mut Towels<'a>, pattern: &'a str) -> usize {
    if let Some(&count) = towels.cache.get(pattern) {
        return count;
    }

    if pattern.is_empty() {
        return 1;
    }

    let mut count = 0;

    for towel in towels.towels {
        if let Some(rest) = pattern.strip_prefix(towel) {
            count += ways(towels, rest);
        }
    }

    towels.cache.insert(pattern, count);

    count
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let (towels, patterns) = input.split("\n\n").collect_tuple().unwrap();

    let towels = towels.split(", ").collect_vec();

    let re = Regex::new(&format!("^({})+$", towels.join("|"))).unwrap();

    let part1 = patterns.lines().filter(|line| re.is_match(line)).count();

    let mut towels = Towels {
        towels: &towels,
        cache: HashMap::new(),
    };

    let part2 = patterns.lines().map(|line| ways(&mut towels, line)).sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day19.txt")) == crate::Solution(6, 16));
    }
}
