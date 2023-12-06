// Generated with ChatGPT 4.
// https://chat.openai.com/share/d3ee1d07-db1b-490a-a756-0651fec38a77

use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let lines: Vec<&str> = input.lines().collect();
    let (times_str, distances_str) = (
        lines[0].split(':').nth(1).unwrap().trim(),
        lines[1].split(':').nth(1).unwrap().trim(),
    );

    // Original question
    let times: Vec<usize> = times_str
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let distances: Vec<usize> = distances_str
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let ways_to_win_per_race: Vec<usize> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            (0..time)
                .filter(|&hold_time| hold_time * (time - hold_time) > distance)
                .count()
        })
        .collect();
    let total_ways_original = ways_to_win_per_race.iter().product();

    // Follow-up question
    let single_time: usize = times_str.replace(" ", "").parse().unwrap();
    let single_distance: usize = distances_str.replace(" ", "").parse().unwrap();
    let ways_to_win_long_race = (0..single_time)
        .filter(|&hold_time| hold_time * (single_time - hold_time) > single_distance)
        .count();

    Solution(total_ways_original, ways_to_win_long_race)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day06.txt")) == crate::Solution(288, 71503));
    }
}
