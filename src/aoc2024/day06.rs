use std::collections::{HashMap, HashSet};

use crate::Solution;
use crate::util::euclid::{Box2, Pt2, Vec2, Vec2Ext, pt2};

struct Grid {
    grid: HashSet<Pt2<isize>>,
    bounds: Box2<isize>,
}

type VisitMap = HashMap<Pt2<isize>, Vec<Vec2<isize>>>;

fn walk(grid: &Grid, mut pos: Pt2<isize>, obstruction: Option<Pt2<isize>>) -> (bool, VisitMap) {
    let mut visited = VisitMap::new();
    let mut dir: Vec2<isize> = Vec2::n();

    loop {
        if visited.contains_key(&pos) && visited[&pos].contains(&dir) {
            return (true, visited);
        }

        visited.entry(pos).or_default().push(dir);

        if !grid.bounds.contains(pos + dir) {
            break;
        } else if grid.grid.contains(&(pos + dir)) || Some(pos + dir) == obstruction {
            dir = dir.rot90r();
        } else {
            pos += dir;
        }
    }

    (false, visited)
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut grid: Grid = Grid {
        grid: HashSet::new(),
        bounds: Box2::new(
            pt2(0, 0),
            pt2(
                input.lines().count() as isize,
                input.lines().next().unwrap().len() as isize,
            ),
        ),
    };
    let mut pos = Pt2::new(0, 0);

    for (i, line) in input.lines().rev().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                grid.grid.insert(Pt2::new(j as isize, i as isize));
            } else if c == '^' {
                pos = Pt2::new(j as isize, i as isize);
            }
        }
    }

    let (_loop, visited) = walk(&grid, pos, None);

    let part1 = visited.len();
    let mut part2 = 0;

    for &pt in visited.keys() {
        if pt == pos {
            continue;
        }
        let (looped, _) = walk(&grid, pos, Some(pt));
        if looped {
            part2 += 1;
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day06.txt")) == crate::Solution(41, 6));
    }
}
