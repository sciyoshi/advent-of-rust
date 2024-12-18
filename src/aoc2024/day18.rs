use itertools::Itertools;
use std::collections::HashSet;

use crate::{
    Solution,
    util::euclid::{Pt2, Pt2Ext, Vec2Ext, pt2},
    utils::extract_integers,
};
use disjoint::DisjointSet;
use pathfinding::directed::astar::astar;

fn part1(dim: isize, bytes: &[Pt2<isize>]) -> usize {
    let grid: HashSet<Pt2<isize>> = bytes.iter().cloned().collect();
    let goal = pt2(dim - 1, dim - 1);

    let path = astar(
        &pt2(0, 0),
        |&pos| {
            pos.nb_ortho()
                .filter(|&nb| {
                    !grid.contains(&nb) && nb.x >= 0 && nb.y >= 0 && nb.x < dim && nb.y < dim
                })
                .map(|nb| (nb, 1))
                .collect_vec()
        },
        |&pos| (pos - goal).norm1(),
        |&pos| pos == goal,
    );

    path.unwrap().0.len() - 1
}

fn part2(dim: isize, bytes: &[Pt2<isize>]) -> Pt2<isize> {
    let mut disjoint = DisjointSet::with_len((dim * dim) as usize);
    let grid: HashSet<Pt2<isize>> = bytes.iter().cloned().collect();

    for y in 0..dim {
        for x in 0..dim {
            let pt = pt2(y, x);
            if !grid.contains(&pt) {
                for nb in pt.nb_ortho() {
                    if nb.x >= 0 && nb.y >= 0 && nb.x < dim && nb.y < dim && !grid.contains(&nb) {
                        disjoint.join((pt.y * dim + pt.x) as usize, (nb.y * dim + nb.x) as usize);
                    }
                }
            }
        }
    }

    for pt in bytes.iter().rev() {
        for nb in pt.nb_ortho() {
            if nb.x >= 0 && nb.y >= 0 && nb.x < dim && nb.y < dim {
                disjoint.join((pt.y * dim + pt.x) as usize, (nb.y * dim + nb.x) as usize);
            }
        }
        if disjoint.is_joined(0, (dim * dim - 1) as usize) {
            return *pt;
        }
    }

    panic!("No solution found");
}

pub fn solve(input: &str) -> Solution<usize, String> {
    let bytes: Vec<Pt2<isize>> = input
        .lines()
        .map(extract_integers::<isize>)
        .map(|v| pt2(v[0], v[1]))
        .collect();

    let pt = part2(71, &bytes);

    Solution(part1(71, &bytes[..1024]), format!("{},{}", pt.x, pt.y))
}

#[cfg(test)]
mod tests {
    use crate::{
        util::euclid::{Pt2, Pt2Ext, Vec2Ext, pt2},
        utils::extract_integers,
    };
    #[test]
    fn test_example() {
        let bytes: Vec<Pt2<isize>> = include_str!("examples/day18.txt")
            .lines()
            .map(extract_integers::<isize>)
            .map(|v| pt2(v[0], v[1]))
            .collect();
        assert!(super::part1(7, &bytes[..12]) == 22);
        assert!(super::part2(7, &bytes) == pt2(6, 1));
    }
}
