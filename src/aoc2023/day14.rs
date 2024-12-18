// Partially generated by ChatGPT
// https://chat.openai.com/share/3e4f4a3f-eafc-4b20-9ff6-8ee1c7c7b9e2

use crate::Solution;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Cube,
    Round,
}

fn parse(grid: &str) -> Vec<Vec<Option<Rock>>> {
    grid.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Some(Rock::Cube),
                    'O' => Some(Rock::Round),
                    '.' => None,
                    _ => panic!("Invalid character in grid"),
                })
                .collect()
        })
        .collect()
}

fn roll_north(grid: &mut [Vec<Option<Rock>>]) {
    let cols = grid.first().map_or(0, Vec::len);
    let mut stops = vec![0; cols];

    for row_idx in 0..grid.len() {
        for col_idx in 0..cols {
            match grid[row_idx][col_idx] {
                Some(Rock::Cube) => {
                    stops[col_idx] = row_idx + 1;
                }
                Some(Rock::Round) => {
                    grid[row_idx][col_idx] = None;
                    grid[stops[col_idx]][col_idx] = Some(Rock::Round);
                    stops[col_idx] += 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_south(grid: &mut [Vec<Option<Rock>>]) {
    let cols = grid.first().map_or(0, Vec::len);
    let mut stops = vec![grid.len() as isize - 1; cols];

    for row_idx in (0..grid.len()).rev() {
        for col_idx in 0..cols {
            match grid[row_idx][col_idx] {
                Some(Rock::Cube) => {
                    stops[col_idx] = row_idx as isize - 1;
                }
                Some(Rock::Round) => {
                    grid[row_idx][col_idx] = None;
                    grid[stops[col_idx] as usize][col_idx] = Some(Rock::Round);
                    stops[col_idx] -= 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_west(grid: &mut [Vec<Option<Rock>>]) {
    let mut stops = vec![0; grid.len()];

    for (row_idx, row) in grid.iter_mut().enumerate() {
        for col_idx in 0..row.len() {
            match row[col_idx] {
                Some(Rock::Cube) => {
                    stops[row_idx] = col_idx + 1;
                }
                Some(Rock::Round) => {
                    row[col_idx] = None;
                    row[stops[row_idx]] = Some(Rock::Round);
                    stops[row_idx] += 1;
                }
                _ => {}
            }
        }
    }
}

fn roll_east(grid: &mut [Vec<Option<Rock>>]) {
    let mut stops = vec![grid[0].len() as isize - 1; grid.len()];

    for (row_idx, row) in grid.iter_mut().enumerate() {
        for col_idx in (0..row.len()).rev() {
            match row[col_idx] {
                Some(Rock::Cube) => {
                    stops[row_idx] = col_idx as isize - 1;
                }
                Some(Rock::Round) => {
                    row[col_idx] = None;
                    row[stops[row_idx] as usize] = Some(Rock::Round);
                    stops[row_idx] -= 1;
                }
                _ => {}
            }
        }
    }
}

fn cycle(grid: &mut [Vec<Option<Rock>>]) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn total_load(grid: &[Vec<Option<Rock>>]) -> usize {
    let total_rows = grid.len();
    let mut load = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for _rock in row.iter().flatten().filter(|&&r| r == Rock::Round) {
            load += total_rows - row_idx;
        }
    }

    load
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut grid = parse(input);

    let mut grid1 = grid.clone();
    roll_north(&mut grid1);
    let part1 = total_load(&grid1);

    let mut cache: HashMap<Vec<Vec<Option<Rock>>>, usize> = HashMap::new();

    for i in 0.. {
        if let Some(previous) = cache.get(&grid) {
            if (1_000_000_000 - previous) % (i - previous) == 0 {
                return Solution(part1, total_load(&grid));
            }
        }
        cache.insert(grid.clone(), i);
        cycle(&mut grid);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day14.txt")) == crate::Solution(136, 64));
    }
}
