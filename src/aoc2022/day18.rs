use std::collections::BTreeSet;

use crate::util::Pt;
use crate::utils::{extract_integers, flood_fill};
use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let pts: BTreeSet<Pt<isize, 3>> = input.lines().map(|l| extract_integers(l).into()).collect();

    let part1 = pts
        .iter()
        .flat_map(|pt| pt.nb_ortho())
        .filter(|pt| !pts.contains(pt))
        .count();

    let exterior: BTreeSet<Pt<isize, 3>> = flood_fill(Pt([0, 0, 0]), |pt| {
        pt.nb_ortho()
            .filter(|nb| {
                nb.0[0] >= -1
                    && nb.0[0] <= 21
                    && nb.0[1] >= -1
                    && nb.0[1] <= 21
                    && nb.0[2] >= -1
                    && nb.0[2] <= 21
                    && !pts.contains(&nb)
            })
            .collect::<Vec<_>>()
            .into_iter()
    })
    .collect();

    let part2 = pts
        .iter()
        .flat_map(|pt| pt.nb_ortho())
        .filter(|pt| !pts.contains(pt) && exterior.contains(pt))
        .count();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5") == crate::Solution(64, 58));
    }
}
