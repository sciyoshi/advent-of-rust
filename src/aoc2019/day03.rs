use crate::utils::Pt;
use crate::Solution;
use std::collections::{BTreeMap, BTreeSet};

fn wire(steps: &str) -> BTreeMap<Pt<isize>, usize> {
    let mut pos = Pt(0, 0);
    let mut delay = 0;
    let mut result = BTreeMap::new();

    for step in steps.split(",") {
        let (dir, dist) = step.split_at(1);
        let dist: usize = dist.parse().unwrap();
        let dir = match dir {
            "U" => Pt::n(),
            "R" => Pt::e(),
            "D" => Pt::s(),
            "L" => Pt::w(),
            _ => panic!("invalid direction"),
        };

        for _ in 0..dist {
            pos += dir;
            delay += 1;
            result.insert(pos, delay);
        }
    }

    result
}

pub fn solve(input: &str) -> Solution<isize, usize> {
    let lines: Vec<_> = input.lines().map(wire).collect();

    let pts1: BTreeSet<&Pt<isize>> = lines[0].keys().collect();
    let pts2: BTreeSet<&Pt<isize>> = lines[1].keys().collect();

    let intersections = pts1.intersection(&pts2);
    let part1 = intersections.clone().map(|pt| pt.norm1()).min().unwrap();
    let part2 = intersections
        .map(|pt| lines[0][pt] + lines[1][pt])
        .min()
        .unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
                == crate::Solution(159, 610)
        );
    }
}
