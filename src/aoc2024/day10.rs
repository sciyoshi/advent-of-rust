use std::collections::{BTreeSet, HashMap};

use petgraph::algo::toposort;
use petgraph::prelude::DiGraphMap;

use crate::Solution;
use crate::util::euclid::{Pt2, Pt2Ext};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut grid: HashMap<(isize, isize), usize> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert((i as isize, j as isize), c.to_digit(10).unwrap() as usize);
        }
    }

    let mut graph = DiGraphMap::new();

    for pt in grid.keys() {
        for nb in Pt2::new(pt.0, pt.1).nb_ortho() {
            let nb = nb.to_tuple();
            if grid.get(&nb) == Some(&(grid[pt] + 1)) {
                graph.add_edge(*pt, nb, 0);
            }
        }
    }

    let mut reach: HashMap<(isize, isize), BTreeSet<(isize, isize)>> = HashMap::new();
    let mut counts: HashMap<(isize, isize), usize> = HashMap::new();
    let mut part1 = 0;
    let mut part2 = 0;

    for pt in toposort(&graph, None).unwrap().iter().rev() {
        if graph.neighbors(*pt).count() == 0 && grid[pt] == 9 {
            reach.insert(*pt, BTreeSet::from([*pt]));
            counts.insert(*pt, 1);
        } else {
            reach.insert(
                *pt,
                graph
                    .neighbors(*pt)
                    .flat_map(|nb| reach[&nb].clone())
                    .collect(),
            );
            counts.insert(*pt, graph.neighbors(*pt).map(|nb| counts[&nb]).sum());
        }

        if grid[pt] == 0 {
            part1 += reach[pt].len();
            part2 += counts[pt];
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day10.txt")) == crate::Solution(36, 81));
    }
}
