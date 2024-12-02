// Generated with ChatGPT 4.

use crate::Solution;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let part1 = input
        .lines()
        .map(|line| {
            // Find the first digit in the line
            let first_digit = line.chars().find(|c| c.is_ascii_digit());

            // Find the last digit in the line
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit());

            match (first_digit, last_digit) {
                (Some(f), Some(l)) => {
                    // Combine them into a two-digit number
                    let number_str = format!("{}{}", f, l);
                    // Parse the number and unwrap it, assuming it will always be a valid number
                    number_str.parse::<usize>().unwrap()
                }
                _ => 0, // Return 0 if any of the digits is not found
            }
        })
        .sum();

    let digit_map = create_digit_map();

    let part2 = input
        .lines()
        .map(|line| {
            let mut digits = extract_digits(line, &digit_map);

            if digits.is_empty() {
                return Err("No digits found");
            }

            digits.sort_by_key(|&(_, pos)| pos);

            let first_digit = digits.first().unwrap().0;
            let last_digit = digits.last().unwrap().0;
            Ok(first_digit * 10 + last_digit)
        })
        .try_fold(0, |acc, res| res.map(|num| acc + num))
        .unwrap();

    Solution(part1, part2)
}

fn create_digit_map() -> HashMap<&'static str, usize> {
    let digits = [
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    digits.iter().cloned().collect()
}

fn extract_digits<'a>(line: &'a str, digit_map: &HashMap<&'a str, usize>) -> Vec<(usize, usize)> {
    digit_map
        .iter()
        .flat_map(|(&word, &num)| line.match_indices(word).map(move |(index, _)| (num, index)))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day01.txt")) == crate::Solution(209, 281));
    }
}
