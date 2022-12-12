use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let (part1, _) = data.iter().fold((0, i32::MAX), |(acc, y), &x| {
        (if x > y { acc + 1 } else { acc }, x)
    });

    let (part2, _) = data.windows(3).fold((0, i32::MAX), |(acc, y), x| {
        let t = x[0] + x[1] + x[2];
        (if t > y { acc + 1 } else { acc }, t)
    });

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("199\n200\n208\n210\n200\n207\n240\n269\n260\n263")
                == crate::Solution(7, 5)
        );
    }
}
