// Generated with ChatGPT 4.
// https://chat.openai.com/share/cd586da5-317f-4863-8b16-a13e12e464d0

use crate::Solution;
use std::collections::{HashMap, HashSet};

fn path_positions(
    grid: &Vec<Vec<char>>,
    adjacency_list: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let start = find_start(grid).expect("Start position not found");
    let mut current = start;

    loop {
        visited.insert(current);

        // Find the next node
        if let Some(neighbors) = adjacency_list.get(&current) {
            let next = neighbors.iter().find(|&&node| !visited.contains(&node));

            match next {
                Some(&node) => current = node,
                None => break,
            }
        } else {
            break;
        }
    }

    visited
}
fn replacement_for_s(
    position: (usize, usize),
    adjacency_list: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> char {
    let neighbors = match adjacency_list.get(&position) {
        Some(neighbors) => neighbors,
        None => return 'S', // If there are no neighbors, return 'S'
    };

    match neighbors.len() {
        2 => {
            let (n1, n2) = (neighbors[0], neighbors[1]);
            if n1.0 == n2.0 {
                '-'
            } else if n1.1 == n2.1 {
                '|'
            } else if (n1.0 < position.0 && n2.1 > position.1)
                || (n2.0 < position.0 && n1.1 > position.1)
            {
                'L'
            } else if (n1.0 < position.0 && n2.1 < position.1)
                || (n2.0 < position.0 && n1.1 < position.1)
            {
                'J'
            } else if (n1.0 > position.0 && n2.1 < position.1)
                || (n2.0 > position.0 && n1.1 < position.1)
            {
                '7'
            } else {
                'F'
            }
        }
        _ => 'S',
    }
}

fn create_grid_copy(
    grid: &Vec<Vec<char>>,
    path_positions: &HashSet<(usize, usize)>,
    adjacency_list: &HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> Vec<Vec<char>> {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &cell)| {
                    if path_positions.contains(&(i, j)) {
                        if cell == 'S' {
                            replacement_for_s((i, j), adjacency_list)
                        } else {
                            cell
                        }
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect()
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn to_adjacency_list(grid: &[Vec<char>]) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut adjacency_list = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let mut neighbors = Vec::new();
            match cell {
                '|' => {
                    if i > 0 {
                        neighbors.push((i - 1, j));
                    }
                    if i < grid.len() - 1 {
                        neighbors.push((i + 1, j));
                    }
                }
                '-' => {
                    if j > 0 {
                        neighbors.push((i, j - 1));
                    }
                    if j < row.len() - 1 {
                        neighbors.push((i, j + 1));
                    }
                }
                'L' => {
                    if i > 0 {
                        neighbors.push((i - 1, j));
                    }
                    if j < row.len() - 1 {
                        neighbors.push((i, j + 1));
                    }
                }
                'J' => {
                    if i > 0 {
                        neighbors.push((i - 1, j));
                    }
                    if j > 0 {
                        neighbors.push((i, j - 1));
                    }
                }
                '7' => {
                    if i < grid.len() - 1 {
                        neighbors.push((i + 1, j));
                    }
                    if j > 0 {
                        neighbors.push((i, j - 1));
                    }
                }
                'F' => {
                    if i < grid.len() - 1 {
                        neighbors.push((i + 1, j));
                    }
                    if j < row.len() - 1 {
                        neighbors.push((i, j + 1));
                    }
                }
                'S' => {
                    if i > 0 && "|F7".contains(grid[i - 1][j]) {
                        neighbors.push((i - 1, j));
                    }
                    if i < grid.len() - 1 && "|JL".contains(grid[i + 1][j]) {
                        neighbors.push((i + 1, j));
                    }
                    if j > 0 && "-FL".contains(grid[i][j - 1]) {
                        neighbors.push((i, j - 1));
                    }
                    if j < row.len() - 1 && "-J7".contains(grid[i][j + 1]) {
                        neighbors.push((i, j + 1));
                    }
                }
                '.' | _ => {}
            }
            adjacency_list.insert((i, j), neighbors);
        }
    }

    adjacency_list
}

fn loop_area(grid: &Vec<Vec<char>>) -> usize {
    let mut total_count = 0;

    for row in grid {
        let mut parity = 0;
        let mut temp_state = None;

        for &cell in row {
            match cell {
                '|' => parity += 1,
                'L' => temp_state = Some('L'),
                '7' => {
                    if temp_state == Some('L') {
                        parity += 1;
                    }
                    temp_state = None;
                }
                'F' => temp_state = Some('F'),
                'J' => {
                    if temp_state == Some('F') {
                        parity += 1;
                    }
                    temp_state = None;
                }
                '.' => {
                    if parity % 2 != 0 {
                        total_count += 1;
                    }
                }
                _ => (),
            }
        }
    }

    total_count
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let adjacency_list = to_adjacency_list(&grid);
    let path = path_positions(&grid, &adjacency_list);
    let part1 = path.len() / 2;
    let copy = create_grid_copy(&grid, &path, &adjacency_list);
    let part2 = loop_area(&copy);
    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day10.txt")) == crate::Solution(80, 10));
    }
}
