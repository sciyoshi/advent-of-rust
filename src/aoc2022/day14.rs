use std::collections::BTreeSet;

use crate::{
    Solution,
    utils::{Pt, extract_integers},
};

fn drop(grid: &BTreeSet<Pt<isize>>, floor: isize, simulate_floor: bool) -> Option<Pt<isize>> {
    let mut pt = Pt(500, 0);

    if grid.contains(&pt) {
        return None;
    }

    while pt.1 <= floor {
        if simulate_floor && pt.1 + 1 == floor {
            return Some(pt);
        } else if !grid.contains(&(pt + Pt::n())) {
            pt += Pt::n();
        } else if !grid.contains(&(pt + Pt::nw())) {
            pt += Pt::nw();
        } else if !grid.contains(&(pt + Pt::ne())) {
            pt += Pt::ne();
        } else {
            return Some(pt);
        }
    }

    None
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut grid: BTreeSet<Pt<isize>> = BTreeSet::new();

    for lines in input.lines().map(|l| {
        extract_integers::<isize>(l)
            .array_chunks::<2>()
            .map(|a| Pt(a[0], a[1]))
            .collect::<Vec<_>>()
    }) {
        for &[p1, p2] in lines.array_windows() {
            let abs = (p2 - p1).norm1();
            let dir = (p2 - p1) / abs;

            for i in 0..=abs {
                grid.insert(p1 + dir * i);
            }
        }
    }

    let floor = grid.iter().map(|pt| pt.1).max().unwrap() + 2;

    let mut part1 = 0;
    let mut grid1 = grid.clone();
    while let Some(pt) = drop(&grid1, floor, false) {
        grid1.insert(pt);
        part1 += 1;
    }

    let mut part2 = 0;
    while let Some(pt) = drop(&grid, floor, true) {
        grid.insert(pt);
        part2 += 1;
    }

    println!("{:?}", grid);
    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9")
                == crate::Solution(24, 93)
        );
    }
}
