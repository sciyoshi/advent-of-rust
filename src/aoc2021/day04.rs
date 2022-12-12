use crate::util::ints;
use crate::Solution;
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

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let rolls = ints(&data[0]);

    let mut boards: Vec<Board> = vec![];
    let mut i = 2;
    while i < data.len() {
        boards.push(Board {
            rows: data[i..i + 5].iter().map(|line| ints(line)).collect(),
        });
        i += 6;
    }

    let scores: Vec<(usize, i64)> = boards.iter().map(|board| board.wins(&rolls)).collect();

    println!(
        "[Part 1] {:?}",
        scores.iter().min_by_key(|&(i, _)| i).unwrap().1
    );
    println!(
        "[Part 2] {:?}",
        scores.iter().max_by_key(|&(i, _)| i).unwrap().1
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
