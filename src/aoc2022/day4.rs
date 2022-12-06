use crate::Solution;

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

pub fn solve(input: &str) -> Solution<usize, usize> {
    let ranges = input
        .lines()
        .map(|l| {
            l.split(&['-', ','])
                .map(str::parse)
                .map(Result::unwrap)
                .collect()
        })
        .map(|l: Vec<i32>| (l[0]..=l[1], l[2]..=l[3]))
        .collect::<Vec<_>>();

    let part1 = ranges
        .iter()
        .filter(|(r1, r2)| range_contains(r1, r2))
        .count();

    let part2 = ranges
        .iter()
        .filter(|(r1, r2)| range_overlaps(r1, r2))
        .count();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day4.txt")) == crate::Solution(2, 4));
    }
}
