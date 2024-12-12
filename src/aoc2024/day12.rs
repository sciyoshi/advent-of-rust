use std::collections::{HashMap, HashSet};

use crate::{
    Solution,
    util::euclid::{Pt2, Pt2Ext, Vec2, Vec2Ext},
};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut grid: HashMap<Pt2<isize>, char> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert(Pt2::new(j as isize, i as isize), c);
        }
    }

    let mut visited = HashSet::new();
    let mut part1 = 0;
    let mut part2 = 0;

    // flood fill starting at each point. find area and perimeter of each contiguous region
    for pt in &grid {
        if visited.contains(pt.0) {
            continue;
        }

        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;
        let plant = grid.get(pt.0).unwrap();

        let mut stack = vec![*pt.0];
        while let Some(p) = stack.pop() {
            if visited.contains(&p) {
                continue;
            }

            visited.insert(p);
            area += 1;

            for n in p.nb_ortho() {
                if grid.get(&n) == Some(plant) {
                    stack.push(n);
                } else {
                    perimeter += 1;
                }
            }

            // determine how many interior and exterior corners this square contributes to the region

            // exterior corners: if e.g. N and W are not part of the region, then NW is a corner
            if grid.get(&(p + Vec2::n())) != Some(plant)
                && grid.get(&(p + Vec2::e())) != Some(plant)
            {
                corners += 1;
            }
            if grid.get(&(p + Vec2::e())) != Some(plant)
                && grid.get(&(p + Vec2::s())) != Some(plant)
            {
                corners += 1;
            }
            if grid.get(&(p + Vec2::s())) != Some(plant)
                && grid.get(&(p + Vec2::w())) != Some(plant)
            {
                corners += 1;
            }
            if grid.get(&(p + Vec2::w())) != Some(plant)
                && grid.get(&(p + Vec2::n())) != Some(plant)
            {
                corners += 1;
            }

            // interior corners: if e.g. N and W are part of the region but NW is not, then NW is a corner
            if grid.get(&(p + Vec2::n())) == Some(plant)
                && grid.get(&(p + Vec2::e())) == Some(plant)
                && grid.get(&(p + Vec2::n() + Vec2::e())) != Some(plant)
            {
                corners += 1;
            }
            if grid.get(&(p + Vec2::e())) == Some(plant)
                && grid.get(&(p + Vec2::s())) == Some(plant)
                && grid.get(&(p + Vec2::e() + Vec2::s())) != Some(plant)
            {
                corners += 1;
            }
            if grid.get(&(p + Vec2::s())) == Some(plant)
                && grid.get(&(p + Vec2::w())) == Some(plant)
                && grid.get(&(p + Vec2::s() + Vec2::w())) != Some(plant)
            {
                corners += 1;
            }
            if grid.get(&(p + Vec2::w())) == Some(plant)
                && grid.get(&(p + Vec2::n())) == Some(plant)
                && grid.get(&(p + Vec2::w() + Vec2::n())) != Some(plant)
            {
                corners += 1;
            }
        }

        if area > 0 {
            part1 += area * perimeter;
            part2 += area * corners;
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day12.txt")) == crate::Solution(1930, 1206));
    }
}
