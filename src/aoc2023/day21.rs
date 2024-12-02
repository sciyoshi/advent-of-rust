use crate::Solution;
use crate::util::euclid::{Pt2, Pt2Ext, pt2};
use std::collections::HashSet;

fn expand(grid: &[Vec<bool>], set: HashSet<Pt2<isize>>) -> HashSet<Pt2<isize>> {
    set.iter()
        .flat_map(|pt| pt.nb_ortho())
        .filter(|pt| {
            !grid[pt.x.rem_euclid(grid.len() as isize) as usize]
                [pt.y.rem_euclid(grid[0].len() as isize) as usize]
        })
        .collect()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let start: Pt2<isize> = pt2(grid.len() as isize / 2, grid[0].len() as isize / 2);

    let mut set = HashSet::from([start]);
    let mut interp = vec![];
    let mut part1 = 0;

    let target = 26501365;

    for i in 1..=(grid.len() * 3).max(64) {
        set = expand(&grid, set);
        if i % grid.len() == target % grid.len() {
            interp.push(set.len());
        }
        if i == 64 {
            part1 = set.len();
        }
    }

    let x = target / grid.len();
    let part2 = interp[0]
        + x * (interp[1] - interp[0] + (x - 1) * (interp[2] - 2 * interp[1] + interp[0]) / 2);

    Solution(part1 as usize, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day21.txt"))
                == crate::Solution(2665, 536899568923058)
        );
    }
}
