use core::panic;
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Solution,
    util::euclid::{Pt2, Vec2, pt2, vec2},
};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Box,
    BoxL,
    BoxR,
    Wall,
}

fn move_robot(
    map: &mut ndarray::Array2<Option<Item>>,
    pos: Pt2<isize>,
    dir: Vec2<isize>,
) -> Pt2<isize> {
    // loop through in direction until either a wall or empty space is hit.
    // if empty space, move robot in that direction and move boxes
    // otherwise, robot does not move.
    let first_pos = pos + dir;
    let mut next_pos = first_pos;

    loop {
        match map[(next_pos.x as usize, next_pos.y as usize)] {
            Some(Item::Wall) => break,
            Some(Item::Box) => {}
            Some(Item::BoxL) => {}
            Some(Item::BoxR) => {}
            None => {
                if next_pos != first_pos {
                    // shift items in map by 1
                    while next_pos != first_pos {
                        map[(next_pos.x as usize, next_pos.y as usize)] =
                            map[((next_pos.x - dir.x) as usize, (next_pos.y - dir.y) as usize)];
                        next_pos -= dir;
                    }
                    map[(first_pos.x as usize, first_pos.y as usize)] = None;
                }
                return first_pos;
            }
        }
        next_pos += dir;
    }

    pos
}

fn move_robot2(
    map: &mut ndarray::Array2<Option<Item>>,
    pos: Pt2<isize>,
    dir: Vec2<isize>,
) -> Pt2<isize> {
    // if left or right, similar as before
    if dir.x == 0 {
        return move_robot(map, pos, dir);
    }

    // otherwise, keep track of a "push set" for each row.
    let mut push_sets: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::new();
    let mut row = pos.x + dir.x;

    push_sets.entry(pos.x).or_default().insert(pos.y);

    loop {
        let last_row = push_sets.get(&(row - dir.x)).unwrap().clone();

        let push_set = push_sets.entry(row).or_default();
        for col in last_row.iter() {
            // if col is BoxL, also add BoxR, and vice-versa
            match map[(row as usize, *col as usize)] {
                Some(Item::BoxL) => {
                    push_set.insert(*col);
                    push_set.insert(col + 1);
                }
                Some(Item::BoxR) => {
                    push_set.insert(*col);
                    push_set.insert(col - 1);
                }
                Some(Item::Wall) => {
                    return pos;
                }
                None => {}
                _ => panic!("Invalid item"),
            }
        }

        if push_set.is_empty() {
            // update map in reverse
            while row != pos.x {
                let push_set = &push_sets[&row];
                let last_push_set = &push_sets[&(row - dir.x)];
                for col in push_set.iter() {
                    map[(row as usize, *col as usize)] = None;
                }
                for col in last_push_set.iter() {
                    map[(row as usize, *col as usize)] =
                        map[((row - dir.x) as usize, (col - dir.y) as usize)];
                }
                row -= dir.x;
            }
            row += dir.x;
            break;
        }

        row += dir.x;
    }

    pt2(row, pos.y)
}

#[allow(dead_code)]
fn print_map(map: &ndarray::Array2<Option<Item>>, pos: Pt2<isize>) {
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if pos == pt2(i as isize, j as isize) {
                print!("@");
            } else {
                print!("{}", match map[(i, j)] {
                    Some(Item::Wall) => '#',
                    Some(Item::Box) => 'O',
                    Some(Item::BoxL) => '[',
                    Some(Item::BoxR) => ']',
                    None => '.',
                });
            }
        }
        println!();
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    // split grid and moves
    let (grid, moves) = input.split("\n\n").collect_tuple().unwrap();

    let width = grid.lines().next().unwrap().len();
    let height = grid.lines().count();

    let moves = moves
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            'v' => vec2(1, 0),
            '^' => vec2(-1, 0),
            '<' => vec2(0, -1),
            '>' => vec2(0, 1),
            _ => panic!("Invalid move"),
        })
        .collect_vec();

    let mut map = ndarray::Array2::<Option<Item>>::default((height, width));
    let mut pos: Pt2<isize> = pt2(0, 0);

    for (i, line) in grid.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[(i, j)] = match c {
                '#' => Some(Item::Wall),
                'O' => Some(Item::Box),
                '@' => {
                    pos = pt2(i as isize, j as isize);
                    None
                }
                '.' => None,
                _ => panic!("Invalid character"),
            };
        }
    }

    for dir in &moves {
        pos = move_robot(&mut map, pos, *dir);
    }

    let mut gps = 0;
    for i in 0..map.shape()[0] {
        for j in 0..map.shape()[1] {
            if let Some(Item::Box) = map[(i, j)] {
                gps += i * 100 + j;
            }
        }
    }

    let mut map2 = ndarray::Array2::<Option<Item>>::default((height, width * 2));

    for (i, line) in grid.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    map2[(i, 2 * j)] = Some(Item::Wall);
                    map2[(i, 2 * j + 1)] = Some(Item::Wall);
                }
                'O' => {
                    map2[(i, 2 * j)] = Some(Item::BoxL);
                    map2[(i, 2 * j + 1)] = Some(Item::BoxR);
                }
                '@' => {
                    pos = pt2(i as isize, 2 * j as isize);
                }
                '.' => {}
                _ => panic!("Invalid character"),
            };
        }
    }

    // print_map(&map2, pos);

    for dir in &moves {
        pos = move_robot2(&mut map2, pos, *dir);
    }

    let mut gps2 = 0;
    for i in 0..map2.shape()[0] {
        for j in 0..map2.shape()[1] {
            if let Some(Item::BoxL) = map2[(i, j)] {
                gps2 += i * 100 + j;
            }
        }
    }

    Solution(gps, gps2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day15.txt")) == crate::Solution(10092, 9021));
    }
}
