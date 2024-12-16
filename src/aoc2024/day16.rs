use std::collections::HashSet;

use crate::Solution;
use crate::util::euclid::{Pt2, Vec2, Vec2Ext, pt2};
use ndarray::Array2;
use pathfinding::directed::astar::astar_bag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pos: Pt2<isize>,
    dir: Vec2<isize>,
}

impl Pos {
    fn successors(&self, grid: &Array2<bool>) -> Vec<(Pos, usize)> {
        let Pos { pos, dir } = *self;
        let mut result = vec![
            (
                Pos {
                    pos,
                    dir: dir.rot90l(),
                },
                1000,
            ),
            (
                Pos {
                    pos,
                    dir: dir.rot90r(),
                },
                1000,
            ),
        ];

        let forward = pos + dir;
        if !grid[(forward.x as usize, forward.y as usize)] {
            result.push((Pos { pos: forward, dir }, 1));
        }

        result
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut start = pt2(0, 0);
    let mut goal = pt2(0, 0);

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid: Array2<bool> = Array2::default((width, height));

    for (i, line) in input.lines().rev().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = pt2(j as isize, i as isize);
                }
                'E' => {
                    goal = pt2(j as isize, i as isize);
                }
                '#' => {
                    grid[(j, i)] = true;
                }
                '.' => {}
                _ => panic!("Invalid character in input"),
            }
        }
    }

    let result = astar_bag(
        &Pos {
            pos: start,
            dir: Vec2::e(),
        },
        |p| p.successors(&grid),
        |p| (p.pos - goal).norm1() as usize,
        |p| p.pos == goal,
    )
    .unwrap();

    let mut positions = HashSet::new();
    for path in result.0 {
        for pos in path {
            positions.insert(pos.pos);
        }
    }

    Solution(result.1, positions.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day16.txt")) == crate::Solution(7036, 45));
    }
}
