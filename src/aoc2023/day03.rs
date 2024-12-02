// Generated with ChatGPT 4.

use crate::Solution;

fn has_valid_adjacent(grid: &[Vec<char>], i: usize, start_col: usize, end_col: usize) -> bool {
    for j in start_col..=end_col {
        if is_adjacent_to_valid_char(grid, i, j) {
            return true;
        }
    }
    false
}

fn is_adjacent_to_valid_char(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (di, dj) in directions {
        if let Some(adjacent_char) = get_char_at(grid, i as isize + di, j as isize + dj) {
            if !adjacent_char.is_ascii_digit() && adjacent_char != '.' {
                return true;
            }
        }
    }
    false
}

fn get_char_at(grid: &[Vec<char>], i: isize, j: isize) -> Option<char> {
    if i < 0 || j < 0 {
        return None;
    }
    let (i, j) = (i as usize, j as usize);
    grid.get(i).and_then(|row| row.get(j)).cloned()
}

type PartNumber = (usize, (usize, usize), (usize, usize));

fn calculate_gear_ratio(
    grid: &[Vec<char>],
    part_numbers: &[PartNumber],
    i: usize,
    j: usize,
) -> usize {
    let mut adjacent_parts = Vec::new();
    for &(part_number, start_pos, end_pos) in part_numbers {
        if is_adjacent_to_star(grid, start_pos, end_pos, i, j) {
            adjacent_parts.push(part_number);
            if adjacent_parts.len() > 2 {
                return 0; // More than two parts adjacent to '*', so not counted
            }
        }
    }

    if adjacent_parts.len() == 2 {
        adjacent_parts[0] * adjacent_parts[1]
    } else {
        0
    }
}

fn is_adjacent_to_star(
    grid: &[Vec<char>],
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    star_i: usize,
    star_j: usize,
) -> bool {
    for i in start_pos.0..=end_pos.0 {
        for j in start_pos.1..=end_pos.1 {
            if is_adjacent_to_valid_char(grid, i, j)
                && ((i as isize - star_i as isize).abs() <= 1)
                && ((j as isize - star_j as isize).abs() <= 1)
            {
                return true;
            }
        }
    }
    false
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Phase 1: Extract part numbers and their positions
    let mut part_numbers = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        let mut j = 0;
        while j < row.len() {
            if row[j].is_ascii_digit() {
                let start = j;
                while j < row.len() && row[j].is_ascii_digit() {
                    j += 1;
                }
                let part_number: usize = row[start..j].iter().collect::<String>().parse().unwrap();
                part_numbers.push((part_number, (i, start), (i, j - 1)));
            }
            j += 1;
        }
    }

    // Phase 2: Summation and Gear Ratio Calculation
    let mut sum = 0;
    let mut gear_ratio = 0;
    for (part_number, start_pos, end_pos) in &part_numbers {
        if has_valid_adjacent(&grid, start_pos.0, start_pos.1, end_pos.1) {
            sum += part_number;
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == '*' {
                gear_ratio += calculate_gear_ratio(&grid, &part_numbers, i, j);
            }
        }
    }

    Solution(sum, gear_ratio)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day03.txt")) == crate::Solution(4361, 467835));
    }
}
