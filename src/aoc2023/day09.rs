// Generated with ChatGPT 4.
// https://chat.openai.com/share/cec0ba6c-3c81-4155-ad7f-17b4ff7463aa

use crate::Solution;

fn find_element(sequence: &[isize]) -> (isize, isize) {
    if sequence.iter().all(|&x| x == 0) {
        return (0, 0);
    }

    if sequence.len() <= 1 {
        return (sequence[0], sequence[0]);
    }

    let differences: Vec<isize> = sequence.windows(2).map(|w| w[1] - w[0]).collect();

    if differences.iter().all(|&x| x == 0) {
        (*sequence.last().unwrap(), sequence[0])
    } else {
        let (next, prev) = find_element(&differences);
        (sequence.last().unwrap() + next, sequence[0] - prev)
    }
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let sequences: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect()
        })
        .collect();

    let (part1, part2) = sequences
        .iter()
        .map(|seq| find_element(seq))
        .fold((0, 0), |(sum_next, sum_prev), (next, prev)| {
            (sum_next + next, sum_prev + prev)
        });

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day09.txt")) == crate::Solution(114, 2));
    }
}
