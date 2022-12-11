use crate::utils::Pt;
use crate::Solution;
use std::collections::BTreeSet;

fn tail_visits(mut rope: Vec<Pt<i32>>, moves: &[(Pt<i32>, usize)]) -> usize {
    let mut visited = BTreeSet::<Pt<i32>>::new();

    for &(d, n) in moves {
        for _ in 0..n {
            rope[0] += d;

            for k in 1..rope.len() {
                let diff = rope[k - 1] - rope[k];
                if diff.normi() > 1 {
                    rope[k].0 += diff.0.signum();
                    rope[k].1 += diff.1.signum();
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let moves = input
        .lines()
        .map(|l| {
            let mut bits = l.split_whitespace();
            let pt: Pt<i32> = match bits.next().unwrap() {
                "D" => Pt::s(),
                "U" => Pt::n(),
                "L" => Pt::w(),
                "R" => Pt::e(),
                _ => panic!("unknown direction"),
            };
            let dist = bits.next().unwrap().parse::<usize>().unwrap();
            (pt, dist)
        })
        .collect::<Vec<_>>();

    let part1 = tail_visits(vec![Pt::zero(); 2], &moves);
    let part2 = tail_visits(vec![Pt::zero(); 10], &moves);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day9-1.txt")) == crate::Solution(13, 1));
        assert!(super::solve(include_str!("examples/day9-2.txt")).1 == 36);
    }
}
