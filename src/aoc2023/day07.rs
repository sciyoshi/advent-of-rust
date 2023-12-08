// Generated with ChatGPT 4.
// https://chat.openai.com/share/60573e3b-0824-4677-bdfb-a394ef7c1e32

use crate::Solution;
use itertools::Itertools;
use std::collections::HashMap;

fn hand_strength(hand: &str, use_joker: bool) -> (usize, usize, usize, usize, usize, usize) {
    let mut card_counts = HashMap::new();
    let mut joker_count = 0;

    let card_values: Vec<usize> = hand
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => {
                if use_joker {
                    joker_count += 1;
                    1
                } else {
                    11
                }
            }
            _ => c.to_digit(10).unwrap() as usize,
        })
        .collect();

    for &value in &card_values {
        if !(use_joker && value == 1) {
            // Skip jokers if using joker rules
            *card_counts.entry(value).or_insert(0) += 1;
        }
    }

    let hand_type = calculate_hand_type(&card_counts, joker_count);

    (
        hand_type,
        card_values[0],
        card_values[1],
        card_values[2],
        card_values[3],
        card_values[4],
    )
}

fn calculate_hand_type(card_counts: &HashMap<usize, usize>, joker_count: usize) -> usize {
    let mut counts = card_counts.clone();
    let mut max_count = *counts.values().max().unwrap_or(&0);
    let mut unique_card_count = counts.len();

    for _ in 0..joker_count {
        if let Some((&card, &count)) = counts.iter().find(|&(_, &count)| count == max_count) {
            counts.insert(card, count + 1);
        } else {
            unique_card_count += 1;
            counts.insert(0, 1); // 0 represents a "fake" card created by a joker
        }

        max_count = *counts.values().max().unwrap();
    }

    match (max_count, unique_card_count) {
        (5, _) => 7,     // Five of a kind (including jokers)
        (4, _) => 6,     // Four of a kind
        (3, 2) => 5,     // Full house
        (3, _) => 4,     // Three of a kind
        (2, 2..=3) => 3, // Two pair (2 pairs, possibly with a joker)
        (2, _) => 2,     // One pair
        _ => 1,          // High card
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let hands_and_bids = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let hand = parts[0];
            let bid: usize = parts[1].parse().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    let calculate_winnings = |use_joker: bool| {
        hands_and_bids
            .iter()
            .map(|&(hand, bid)| (hand_strength(hand, use_joker), bid))
            .sorted_by_key(|&(strength, _)| strength)
            .enumerate()
            .map(|(rank, (_, bid))| bid * (rank + 1))
            .sum()
    };

    let total_winnings_normal = calculate_winnings(false);
    let total_winnings_joker = calculate_winnings(true);

    Solution(total_winnings_normal, total_winnings_joker)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day07.txt")) == crate::Solution(6440, 5905));
    }
}
