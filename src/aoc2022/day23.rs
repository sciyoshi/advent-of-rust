use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use counter::Counter;

use crate::{utils::Pt, Solution};

fn round(elves: HashSet<Pt<isize>>, round: usize) -> (HashSet<Pt<isize>>, bool) {
    let mut checks = vec![
        ([Pt::n(), Pt::ne(), Pt::nw()], Pt::n()),
        ([Pt::s(), Pt::se(), Pt::sw()], Pt::s()),
        ([Pt::w(), Pt::nw(), Pt::sw()], Pt::w()),
        ([Pt::e(), Pt::ne(), Pt::se()], Pt::e()),
    ];

    checks.rotate_left(round % 4);

    let mut moves = HashMap::new();

    for &elf in elves.iter() {
        if elf.nb8().iter().all(|p| !elves.contains(p)) {
            continue;
        }

        for &(check, dir) in &checks {
            if check.into_iter().all(|d| !elves.contains(&(elf + d))) {
                moves.insert(elf, elf + dir);
                break;
            }
        }
    }

    let targets: Counter<&Pt<isize>, usize> = moves.values().collect();
    let mut new_elves = HashSet::new();
    let mut any_moved = false;

    for &elf in elves.iter() {
        if let Some(&target) = moves.get(&elf) && targets[&target] == 1 {
            new_elves.insert(target);
            any_moved = true;
        } else {
            new_elves.insert(elf);
        }
    }

    (new_elves, any_moved)
}

pub fn solve(input: &str) -> Solution<isize, usize> {
    let mut elves = HashSet::new();

    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Pt(x as isize, y as isize));
            }
        }
    }

    for i in 0..10 {
        (elves, _) = round(elves, i);
    }

    let (xmin, xmax) = elves.iter().map(|p| p.0).minmax().into_option().unwrap();
    let (ymin, ymax) = elves.iter().map(|p| p.1).minmax().into_option().unwrap();

    let part1 = (xmax - xmin + 1) * (ymax - ymin + 1) - elves.len() as isize;

    for i in 10.. {
        let (new_elves, any_moved) = round(elves, i);
        if !any_moved {
            return Solution(part1, i + 1);
        }
        elves = new_elves;
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day23.txt")) == crate::Solution(110, 20));
    }
}
