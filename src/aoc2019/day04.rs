use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    Solution(0, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
