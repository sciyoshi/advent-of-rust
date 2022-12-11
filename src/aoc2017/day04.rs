use crate::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_whitespace().map(|w| w.to_string()).collect())
        .collect();

    // Count lines where all words are unique
    let count1 = lines
        .iter()
        .filter(|line| line.iter().unique().count() == line.len())
        .count();

    // Count lines where all sorted words are unique (to detect anagrams)
    let count2 = lines
        .iter()
        .filter(|line| {
            let words: Vec<_> = line
                .iter()
                .map(|w| w.chars().sorted().collect::<Vec<_>>())
                .collect();
            words.iter().unique().count() == words.len()
        })
        .count();

    Solution(count1, count2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa").0 == 2);
        assert!(super::solve("abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio").1 == 3);
    }
}
