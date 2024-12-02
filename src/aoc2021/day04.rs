use crate::Solution;
use crate::utils::extract_integers;
use std::iter::Iterator;

struct Board {
    rows: Vec<Vec<i64>>,
}

impl Board {
    fn check(&self, rolls: &[i64]) -> bool {
        for i in 0..5 {
            if (0..5).map(|j| self.rows[i][j]).all(|v| rolls.contains(&v)) {
                return true;
            }
            if (0..5).map(|j| self.rows[j][i]).all(|v| rolls.contains(&v)) {
                return true;
            }
        }

        false
    }

    fn wins(&self, rolls: &[i64]) -> (usize, i64) {
        for i in 1..rolls.len() {
            if self.check(&rolls[..i]) {
                let mut score = 0;
                for row in &self.rows {
                    for col in row {
                        if !rolls[..i].contains(col) {
                            score += col;
                        }
                    }
                }
                return (i, rolls[i - 1] * score);
            }
        }

        (0, 0)
    }
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let data: Vec<_> = input.lines().collect();

    let rolls = extract_integers(&data[0]);

    let mut boards: Vec<Board> = vec![];
    let mut i = 2;
    while i < data.len() {
        boards.push(Board {
            rows: data[i..i + 5]
                .iter()
                .map(|line| extract_integers(line))
                .collect(),
        });
        i += 6;
    }

    let scores: Vec<(usize, i64)> = boards.iter().map(|board| board.wins(&rolls)).collect();

    let part1 = scores.iter().min_by_key(|&(i, _)| i).unwrap().1;
    let part2 = scores.iter().max_by_key(|&(i, _)| i).unwrap().1;

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day04.txt")) == crate::Solution(4512, 1924));
    }
}
