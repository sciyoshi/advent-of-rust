use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::Solution;
use crate::util::euclid::{Box2, Pt2, pt2};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut grid: HashMap<char, Vec<Pt2<isize>>> = HashMap::new();
    let mut antinodes: HashSet<Pt2<isize>> = HashSet::new();
    let mut resonant_antinodes: HashSet<Pt2<isize>> = HashSet::new();

    let bounds: Box2<isize> = Box2::from_size(
        [
            input.lines().count() as isize,
            input.lines().next().unwrap().len() as isize,
        ]
        .into(),
    );

    for (j, line) in input.lines().rev().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c != '.' {
                grid.entry(c).or_default().push(pt2(i as isize, j as isize));
            }
        }
    }

    for (_c, pts) in grid.iter() {
        for (&pt1, &pt2) in pts.iter().tuple_combinations() {
            if bounds.contains(pt2 + (pt2 - pt1)) {
                antinodes.insert(pt2 + (pt2 - pt1));
            }
            if bounds.contains(pt1 + (pt1 - pt2)) {
                antinodes.insert(pt1 + (pt1 - pt2));
            }

            let mut pt = pt1 + (pt2 - pt1);
            while bounds.contains(pt) {
                resonant_antinodes.insert(pt);
                pt += pt2 - pt1;
            }

            let mut pt = pt1;
            while bounds.contains(pt) {
                resonant_antinodes.insert(pt);
                pt -= pt2 - pt1;
            }
        }
    }

    Solution(antinodes.len(), resonant_antinodes.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day08.txt")) == crate::Solution(14, 34));
    }
}
