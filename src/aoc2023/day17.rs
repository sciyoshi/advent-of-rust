// Generated with ChatGPT
// https://chat.openai.com/share/8d07596e-838f-407d-9d38-1c4994bd9a1f

use crate::Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
    direction: (i32, i32),
    steps: i32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: Position,
    f_score: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(pos: &Position, goal: &(usize, usize)) -> i32 {
    (goal.0 as i32 - pos.x as i32).abs() + (goal.1 as i32 - pos.y as i32).abs()
}

fn reconstruct_path(
    came_from: &HashMap<Position, Position>,
    mut current: Position,
) -> Vec<Position> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}

fn a_star_search(
    grid: &Vec<Vec<i32>>,
    min_steps: i32,
    max_steps: i32,
) -> Option<(i32, Vec<Position>)> {
    let goal = (grid.len() - 1, grid[0].len() - 1);
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    let start = Position {
        x: 0,
        y: 0,
        direction: (0, 0),
        steps: 0,
    };
    g_score.insert(start, 0);
    open_set.push(Node {
        position: start,
        f_score: heuristic(&start, &goal),
    });

    while let Some(Node {
        position: current, ..
    }) = open_set.pop()
    {
        if (current.x, current.y) == goal && current.steps >= min_steps {
            let path = reconstruct_path(&came_from, current);
            return Some((g_score[&current], path));
        }

        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_dir = (dx, dy);
            // Prevent moving backwards
            if new_dir == (-current.direction.0, -current.direction.1) {
                continue;
            }

            let next = Position {
                x: (current.x as i32 + dx) as usize,
                y: (current.y as i32 + dy) as usize,
                direction: new_dir,
                steps: if current.direction == new_dir {
                    current.steps + 1
                } else {
                    1
                },
            };

            if next.x >= grid.len() || next.y >= grid[0].len() {
                continue;
            }

            // Enforce minimum and maximum step constraints
            if next.steps > max_steps
                || (current.direction != new_dir
                    && current.direction != (0, 0)
                    && current.steps < min_steps)
            {
                continue;
            }

            let tentative_g_score = g_score[&current] + grid[next.x][next.y];
            if tentative_g_score < *g_score.get(&next).unwrap_or(&i32::MAX) {
                came_from.insert(next, current);
                g_score.insert(next, tentative_g_score);
                let f_score = tentative_g_score + heuristic(&next, &goal);
                open_set.push(Node {
                    position: next,
                    f_score,
                });
            }
        }
    }

    None
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let part1 = a_star_search(&grid, 0, 3).unwrap();
    let part2 = a_star_search(&grid, 4, 10).unwrap();

    Solution(part1.0 as usize, part2.0 as usize)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day17.txt")) == crate::Solution(102, 94));
    }
}
