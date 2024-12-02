use crate::Solution;
use crate::utils::extract_integers;
use std::iter::Iterator;

pub fn solve(input: &str) -> Solution<i64, i64> {
    let data: Vec<i64> = extract_integers(input);

    let max = *data.iter().max().unwrap();

    let part1 = (0..=max)
        .map(|n| data.iter().map(|c| (c - n).abs()).sum::<i64>())
        .min()
        .unwrap();

    let part2 = (0..=max)
        .map(|n| {
            data.iter()
                .map(|c| (c - n).abs() * ((c - n).abs() + 1) / 2)
                .sum::<i64>()
        })
        .min()
        .unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("16,1,2,0,4,2,7,1,2,14") == crate::Solution(37, 168));
    }
}
