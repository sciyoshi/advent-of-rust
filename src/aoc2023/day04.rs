// Generated with ChatGPT 4.

use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let cards: Vec<(Vec<usize>, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            let winning_numbers = parts[0]
                .split_whitespace()
                .skip(2) // Skip "Card X:"
                .map(|num| num.parse().unwrap())
                .collect();
            let player_numbers = parts[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (winning_numbers, player_numbers)
        })
        .collect();

    // Part 1: Calculate total points
    let total_points = cards
        .iter()
        .map(|(winning, player)| {
            let matches = winning.iter().filter(|&&num| player.contains(&num)).count();
            if matches > 0 {
                2usize.pow(matches as u32 - 1)
            } else {
                0
            }
        })
        .sum();

    // Part 2: Calculate total number of scratchcards
    let mut total_cards_count = vec![1; cards.len()];
    for (i, (winning, _)) in cards.iter().enumerate() {
        let num_wins = winning
            .iter()
            .filter(|&&num| cards[i].1.contains(&num))
            .count();
        for j in i + 1..std::cmp::min(i + 1 + num_wins, cards.len()) {
            total_cards_count[j] += total_cards_count[i];
        }
    }

    let total_scratchcards = total_cards_count.iter().sum();

    Solution(total_points, total_scratchcards)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day04.txt")) == crate::Solution(13, 30));
    }
}
