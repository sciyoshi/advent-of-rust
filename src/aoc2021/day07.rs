use crate::util::ints;
use crate::Solution;
use std::iter::Iterator;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = ints(io::stdin().lock().lines().next().unwrap().unwrap().as_str());

    let max = *data.iter().max().unwrap();

    let part1 = (0..=max)
        .map(|n| data.iter().map(|c| (c - n).abs()).sum::<i64>())
        .min()
        .unwrap();

    println!("[Part 1] {:?}", part1);

    let part2 = (0..=max)
        .map(|n| {
            data.iter()
                .map(|c| (c - n).abs() * ((c - n).abs() + 1) / 2)
                .sum::<i64>()
        })
        .min()
        .unwrap();

    println!("[Part 2] {:?}", part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
