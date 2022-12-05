use crate::utils::extract_integers;
use std::io::stdin;

fn range_contains(
    range1: &std::ops::RangeInclusive<i32>,
    range2: &std::ops::RangeInclusive<i32>,
) -> bool {
    range1.start() >= range2.start() && range1.end() <= range2.end()
        || range2.start() >= range1.start() && range2.end() <= range1.end()
}

fn range_overlaps(
    range1: &std::ops::RangeInclusive<i32>,
    range2: &std::ops::RangeInclusive<i32>,
) -> bool {
    range1.start() <= range2.end() && range1.end() >= range2.start()
}

pub fn solve() {
    let ranges = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| extract_integers(s.as_str()))
        .map(|l| (l[0]..=l[1], l[2]..=l[3]))
        .collect::<Vec<_>>();

    let part1 = ranges
        .iter()
        .filter(|(r1, r2)| range_contains(r1, r2))
        .count() as u32;

    let part2 = ranges
        .iter()
        .filter(|(r1, r2)| range_overlaps(r1, r2))
        .count() as u32;

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
