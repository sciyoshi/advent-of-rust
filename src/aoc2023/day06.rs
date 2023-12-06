// Generated with ChatGPT 4.
// https://chat.openai.com/share/0f59a9a3-cef2-4556-abae-0dfa670ba41a

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
    let mut total_ways_original = 1;
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let root1 = (time as f64 - (time.pow(2) as f64 - 4.0 * distance as f64).sqrt()) / 2.0;
        let root2 = (time as f64 + (time.pow(2) as f64 - 4.0 * distance as f64).sqrt()) / 2.0;
        total_ways_original *= (root2.ceil() as usize - root1.floor() as usize - 1).max(0);
    }

    // Follow-up question
    let single_time: usize = times_str.replace(" ", "").parse().unwrap();
    let single_distance: usize = distances_str.replace(" ", "").parse().unwrap();
    let root1 = (single_time as f64
        - (single_time.pow(2) as f64 - 4.0 * single_distance as f64).sqrt())
        / 2.0;
    let root2 = (single_time as f64
        + (single_time.pow(2) as f64 - 4.0 * single_distance as f64).sqrt())
        / 2.0;
    let ways_to_win_long_race = (root2.ceil() as usize - root1.floor() as usize - 1).max(0);

    Solution(total_ways_original, ways_to_win_long_race)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day06.txt")) == crate::Solution(288, 71503));
    }
}
