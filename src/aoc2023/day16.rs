// Generated with ChatGPT
// https://chat.openai.com/share/af18711d-74a8-42fd-aaf9-a68e2170cdb7

use crate::Solution;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
}

fn energize_tiles(
    grid: &[Vec<char>],
    start_position: (usize, usize),
    start_direction: Direction,
) -> usize {
    let mut beams = VecDeque::new();
    let start_beam = Beam {
        position: start_position,
        direction: start_direction,
    };
    beams.push_back(start_beam);

    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    while let Some(beam) = beams.pop_front() {
        if !visited.insert(beam) {
            continue;
        }

        let Beam {
            mut position,
            mut direction,
        } = beam;

        while let Some(tile) = grid.get(position.0).and_then(|row| row.get(position.1)) {
            energized.insert(position);
            match *tile {
                '.' => (),
                '/' => {
                    direction = match direction {
                        Direction::Right => Direction::Up,
                        Direction::Left => Direction::Down,
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                    }
                }
                '\\' => {
                    direction = match direction {
                        Direction::Right => Direction::Down,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                    }
                }
                '|' => {
                    if direction == Direction::Left || direction == Direction::Right {
                        beams.push_back(Beam {
                            position,
                            direction: Direction::Up,
                        });
                        beams.push_back(Beam {
                            position,
                            direction: Direction::Down,
                        });
                        break;
                    }
                }
                '-' => {
                    if direction == Direction::Up || direction == Direction::Down {
                        beams.push_back(Beam {
                            position,
                            direction: Direction::Left,
                        });
                        beams.push_back(Beam {
                            position,
                            direction: Direction::Right,
                        });
                        break;
                    }
                }
                _ => panic!("Invalid tile encountered"),
            }

            position = match direction {
                Direction::Up => (position.0.wrapping_sub(1), position.1),
                Direction::Down => (position.0 + 1, position.1),
                Direction::Left => (position.0, position.1.wrapping_sub(1)),
                Direction::Right => (position.0, position.1 + 1),
            };
        }
    }

    energized.len()
}

fn max_energized_tiles(grid: &[Vec<char>]) -> usize {
    let mut max_energized = 0;

    let height = grid.len();
    let width = grid[0].len();

    // Top and bottom rows
    for x in 0..width {
        max_energized = max_energized.max(energize_tiles(grid, (0, x), Direction::Down)); // Top row, heading down
        max_energized = max_energized.max(energize_tiles(grid, (height - 1, x), Direction::Up));
        // Bottom row, heading up
    }

    // Left and right columns
    for y in 0..height {
        max_energized = max_energized.max(energize_tiles(grid, (y, 0), Direction::Right)); // Left column, heading right
        max_energized = max_energized.max(energize_tiles(grid, (y, width - 1), Direction::Left));
        // Right column, heading left
    }

    max_energized
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let part1 = energize_tiles(&lines, (0, 0), Direction::Right);
    let part2 = max_energized_tiles(&lines);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day16.txt")) == crate::Solution(46, 51));
    }
}
