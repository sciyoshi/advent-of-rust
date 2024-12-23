use crate::{
    Solution,
    util::euclid::{Pt2, pt2},
};
use counter::Counter;
use itertools::Itertools;
use std::fmt::{self, Display};

fn numpad_pos(c: char) -> Pt2<u8> {
    match c {
        '0' => pt2(1, 0),
        'A' => pt2(2, 0),
        '1' => pt2(0, 1),
        '2' => pt2(1, 1),
        '3' => pt2(2, 1),
        '4' => pt2(0, 2),
        '5' => pt2(1, 2),
        '6' => pt2(2, 2),
        '7' => pt2(0, 3),
        '8' => pt2(1, 3),
        '9' => pt2(2, 3),
        _ => panic!("Invalid position: {}", c),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Key::Left => write!(f, "<"),
            Key::Right => write!(f, ">"),
            Key::Up => write!(f, "^"),
            Key::Down => write!(f, "v"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Steps {
    key: Key,
    count: u8,
}

impl Steps {
    fn up(count: u8) -> Self {
        Self {
            key: Key::Up,
            count,
        }
    }

    fn down(count: u8) -> Self {
        Self {
            key: Key::Down,
            count,
        }
    }

    fn left(count: u8) -> Self {
        Self {
            key: Key::Left,
            count,
        }
    }

    fn right(count: u8) -> Self {
        Self {
            key: Key::Right,
            count,
        }
    }
}

impl Display for Steps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.count {
            write!(f, "{}", self.key)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Group {
    step1: Option<Steps>,
    step2: Option<Steps>,
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(steps) = self.step1 {
            write!(f, "{}", steps)?;
        }
        if let Some(steps) = self.step2 {
            write!(f, "{}", steps)?;
        }
        write!(f, "A")
    }
}

impl Group {
    fn presses(&self) -> usize {
        let mut count = 1;
        if let Some(steps) = self.step1 {
            count += steps.count as usize;
        }
        if let Some(steps) = self.step2 {
            count += steps.count as usize;
        }
        count
    }

    fn empty() -> Self {
        Self {
            step1: None,
            step2: None,
        }
    }

    fn one(step: Steps) -> Self {
        Self {
            step1: Some(step),
            step2: None,
        }
    }

    fn two(step1: Steps, step2: Steps) -> Self {
        if step1.count == 0 {
            Self::one(step2)
        } else if step2.count == 0 {
            Self::one(step1)
        } else {
            Self {
                step1: Some(step1),
                step2: Some(step2),
            }
        }
    }

    fn numpad(pos1: char, pos2: char) -> Self {
        let pos = numpad_pos(pos1);
        let next_pos = numpad_pos(pos2);

        if pos.y == 0 && next_pos.x == 0 {
            Group::two(
                Steps::up(next_pos.y - pos.y),
                Steps::left(pos.x - next_pos.x),
            )
        } else if pos.x == 0 && next_pos.y == 0 {
            Group::two(
                Steps::right(next_pos.x - pos.x),
                Steps::down(pos.y - next_pos.y),
            )
        } else if next_pos.x < pos.x {
            Group::two(
                Steps::left(pos.x - next_pos.x),
                if next_pos.y > pos.y {
                    Steps::up(next_pos.y - pos.y)
                } else {
                    Steps::down(pos.y - next_pos.y)
                },
            )
        } else {
            Group::two(
                if next_pos.y > pos.y {
                    Steps::up(next_pos.y - pos.y)
                } else {
                    Steps::down(pos.y - next_pos.y)
                },
                if next_pos.x > pos.x {
                    Steps::right(next_pos.x - pos.x)
                } else {
                    Steps::left(pos.x - next_pos.x)
                },
            )
        }
    }

    fn steps(pos1: Option<Key>, pos2: Option<Key>) -> Self {
        match (pos1, pos2) {
            // left 2
            (Some(Key::Right), Some(Key::Left)) => Self::one(Steps::left(2)),

            // down 1 left 2
            (None, Some(Key::Left)) => Self::two(Steps::down(1), Steps::left(2)),

            // left 1 up 1
            (Some(Key::Right), Some(Key::Up)) => Self::two(Steps::left(1), Steps::up(1)),

            // left 1
            (None, Some(Key::Up))
            | (Some(Key::Right), Some(Key::Down))
            | (Some(Key::Down), Some(Key::Left)) => Self::one(Steps::left(1)),

            // left 1 down 1
            (None, Some(Key::Down)) | (Some(Key::Up), Some(Key::Left)) => {
                Self::two(Steps::left(1), Steps::down(1))
            }

            // up 1
            (Some(Key::Right), None) | (Some(Key::Down), Some(Key::Up)) => Self::one(Steps::up(1)),

            // down 1
            (None, Some(Key::Right)) | (Some(Key::Up), Some(Key::Down)) => {
                Self::one(Steps::down(1))
            }

            // right 1 up 1
            (Some(Key::Left), Some(Key::Up)) => Self::two(Steps::right(1), Steps::up(1)),

            // up 1 right 1
            (Some(Key::Down), None) => Self::two(Steps::up(1), Steps::right(1)),

            // right 1
            (Some(Key::Up), None)
            | (Some(Key::Down), Some(Key::Right))
            | (Some(Key::Left), Some(Key::Down)) => Self::one(Steps::right(1)),

            // down 1 right 1
            (Some(Key::Up), Some(Key::Right)) => Self::two(Steps::down(1), Steps::right(1)),

            // right 2 up 1
            (Some(Key::Left), None) => Self::two(Steps::right(2), Steps::up(1)),

            // right 2
            (Some(Key::Left), Some(Key::Right)) => Self::one(Steps::right(2)),

            (None, None)
            | (Some(Key::Up), Some(Key::Up))
            | (Some(Key::Down), Some(Key::Down))
            | (Some(Key::Left), Some(Key::Left))
            | (Some(Key::Right), Some(Key::Right)) => Self::empty(),
        }
    }
}

#[memoize::memoize]
fn dpad_steps(input: Group) -> Counter<Group> {
    let mut result = Counter::new();

    match (input.step1, input.step2) {
        (None, None) => {
            result[&Group::empty()] += 1;
        }
        (Some(step1), None) => {
            result[&Group::steps(None, Some(step1.key))] += 1;
            result[&Group::empty()] += step1.count as usize - 1;
            result[&Group::steps(Some(step1.key), None)] += 1;
        }
        (Some(step1), Some(step2)) => {
            result[&Group::steps(None, Some(step1.key))] += 1;
            result[&Group::empty()] += step1.count as usize - 1;
            result[&Group::steps(Some(step1.key), Some(step2.key))] += 1;
            result[&Group::empty()] += step2.count as usize - 1;
            result[&Group::steps(Some(step2.key), None)] += 1;
        }
        _ => panic!(),
    }

    result
}

fn step_counter(input: Counter<Group>) -> Counter<Group> {
    let mut result = Counter::new();

    for (group, count) in input {
        let steps = dpad_steps(group);
        for (step, step_count) in steps {
            result[&step] += count * step_count;
        }
    }

    result
}

fn code_presses(code: &str, robots: usize) -> usize {
    let mut counter = Counter::new();

    for (&pos1, &pos2) in code.chars().collect_vec().iter().circular_tuple_windows() {
        counter[&Group::numpad(pos1, pos2)] += 1;
    }

    for _ in 0..robots {
        counter = step_counter(counter);
    }

    counter.iter().map(|(k, v)| k.presses() * v).sum()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let part1: usize = input
        .lines()
        .map(|code| {
            let count: usize = code_presses(code, 2);
            let numeric: usize = code[0..3].parse().unwrap();

            count * numeric
        })
        .sum();

    let part2: usize = input
        .lines()
        .map(|code| {
            let count: usize = code_presses(code, 25);
            let numeric: usize = code[0..3].parse().unwrap();

            count * numeric
        })
        .sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day21.txt"))
                == crate::Solution(126384, 153424334299654)
        );
    }
}
