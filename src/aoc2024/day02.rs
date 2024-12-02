use crate::{Solution, utils::extract_integers};

fn safe(report: &[usize], skip: Option<usize>) -> bool {
    let mut index = if skip == Some(0) { 2 } else { 1 };
    let mut item = if skip == Some(0) {
        report[1]
    } else {
        report[0]
    };
    let mut direction: Option<bool> = None;

    while index < report.len() {
        if skip == Some(index) {
            index += 1;
            continue;
        }

        if direction.is_none() {
            direction = Some(report[index] > item);
        } else if direction != Some(report[index] > item) {
            return false;
        }

        if !(1..=3).contains(&report[index].abs_diff(item)) {
            return false;
        }

        item = report[index];
        index += 1;
    }

    return true;
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let reports: Vec<Vec<usize>> = input
        .lines()
        .map(|line| extract_integers::<usize>(line))
        .collect();

    let part1 = reports.iter().filter(|report| safe(report, None)).count();

    let part2 = reports
        .iter()
        .filter(|report| (0..report.len()).any(|index| safe(report, Some(index))))
        .count();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day02.txt")) == crate::Solution(2, 4));
    }
}
