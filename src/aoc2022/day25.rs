use num::Integer;

use crate::Solution;

fn snafu_from_str(value: &str) -> i64 {
    value
        .chars()
        .map(|c| match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        })
        .fold(0, |acc, v| 5 * acc + v)
}

fn snafu_to_str(mut value: i64) -> String {
    let mut result = vec![];

    while value > 0 {
        let (div, rem) = (value + 2).div_rem(&5);
        result.push(match rem - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        });
        value = div;
    }

    result.into_iter().rev().collect()
}

pub fn solve(input: &str) -> Solution<String, usize> {
    Solution(snafu_to_str(input.lines().map(snafu_from_str).sum()), 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day25.txt"))
                == crate::Solution("2=-1=0".to_string(), 0)
        );
    }
}
