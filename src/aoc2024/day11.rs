use crate::{Solution, utils::extract_integers};
use memoize::memoize;

#[memoize]
fn stones(n: usize, steps: usize) -> usize {
    if steps == 0 {
        1
    } else if n == 0 {
        stones(1, steps - 1)
    } else {
        let digits = n.checked_ilog10().unwrap() + 1;

        if digits % 2 == 0 {
            stones(n % 10usize.pow(digits / 2), steps - 1)
                + stones(n / 10usize.pow(digits / 2), steps - 1)
        } else {
            stones(n * 2024, steps - 1)
        }
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let start: Vec<usize> = extract_integers(input);

    let part1 = start.iter().map(|&n| stones(n, 25)).sum::<usize>();
    let part2 = start.iter().map(|&n| stones(n, 75)).sum::<usize>();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day11.txt"))
                == crate::Solution(55312, 65601038650482)
        );
    }
}
