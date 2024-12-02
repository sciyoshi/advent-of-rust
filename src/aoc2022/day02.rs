use crate::Solution;

pub fn solve(input: &str) -> Solution<i32, i32> {
    let mut result = Solution(0, 0);

    for line in input.lines() {
        let (score1, score2) = match line {
            "A X" => (1 + 3, 3),
            "A Y" => (2 + 6, 1 + 3),
            "A Z" => (3, 2 + 6),
            "B X" => (1, 1),
            "B Y" => (2 + 3, 2 + 3),
            "B Z" => (3 + 6, 3 + 6),
            "C X" => (1 + 6, 2),
            "C Y" => (2, 3 + 3),
            "C Z" => (3 + 3, 1 + 6),
            _ => panic!("invalid line"),
        };

        result.0 += score1;
        result.1 += score2;
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day2.txt")) == crate::Solution(15, 12));
    }
}
