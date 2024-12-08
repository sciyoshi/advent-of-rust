use std::collections::BTreeSet;

use crate::{Solution, utils::extract_integers};

fn check(lhs: usize, rhs: &[usize], concat: bool) -> bool {
    let result = rhs[1..].iter().fold(BTreeSet::from([rhs[0]]), |acc, &r| {
        acc.iter()
            .flat_map(|e| {
                if concat {
                    vec![
                        e + r,
                        e * r,
                        e * 10usize.pow(r.checked_ilog10().unwrap() + 1) + r,
                    ]
                } else {
                    vec![e + r, e * r]
                }
            })
            .filter(|&e| e <= lhs)
            .collect()
    });

    result.contains(&lhs)
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let equations: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|l| {
            let ints = extract_integers(l);
            (ints[0], ints[1..].to_vec())
        })
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for (lhs, rhs) in &equations {
        if check(*lhs, rhs, false) {
            part1 += lhs;
        }
        if check(*lhs, rhs, true) {
            part2 += lhs;
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day07.txt")) == crate::Solution(3749, 11387));
    }
}
