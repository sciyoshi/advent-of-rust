use crate::utils::extract_integers;
use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<usize> = extract_integers(input);

    let mut counts = (0..=8)
        .map(|i| data.iter().filter(|&n| *n == i).count())
        .collect::<Vec<_>>();

    let mut part1 = 0;

    for i in 1..=256 {
        counts.rotate_left(1);
        counts[6] += counts[8];

        if i == 80 {
            part1 = counts.iter().sum::<usize>();
        }
    }

    let part2 = counts.iter().sum::<usize>();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("3,4,3,1,2") == crate::Solution(5934, 26984457539));
    }
}
