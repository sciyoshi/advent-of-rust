use crate::utils::extract_integers;
use crate::Solution;
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = input.lines().map(|line| extract_integers(line)).collect();

    let mut points: HashMap<(i64, i64), u64> = HashMap::new();

    for l in &data {
        let (x1, y1, x2, y2) = (l[0], l[1], l[2], l[3]);

        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                let count = points.entry((x1, y)).or_insert(0);
                *count += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                let count = points.entry((x, y1)).or_insert(0);
                *count += 1;
            }
        }
    }

    let part1 = points.values().filter(|c| **c > 1).count();

    for l in &data {
        let (x1, y1, x2, y2) = (l[0], l[1], l[2], l[3]);

        if x1 != x2 && y1 != y2 {
            let dist = (x2 - x1).abs();
            let dx = (x2 - x1) / dist;
            let dy = (y2 - y1) / dist;

            for i in 0..=dist {
                let count = points.entry((x1 + i * dx, y1 + i * dy)).or_insert(0);
                *count += 1;
            }
        }
    }

    let part2 = points.values().filter(|c| **c > 1).count();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day05.txt")) == crate::Solution(5, 12));
    }
}
