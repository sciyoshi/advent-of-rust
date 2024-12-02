use crate::Solution;
use bit_vec::BitVec;
use nom::{
    IResult, bytes::complete::tag, character::complete::anychar, character::complete::u64,
    sequence::preceded,
};
use std::collections::HashMap;
use std::ops::Index;
use std::str::Lines;

#[derive(Debug)]
struct Tape {
    left: BitVec,
    right: BitVec,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Tape {
    fn new() -> Self {
        Tape {
            left: BitVec::new(),
            right: BitVec::new(),
        }
    }

    fn grow(&mut self, i: isize) {
        if i >= self.right.len() as isize {
            self.right.reserve(1024);
            self.right.grow(i as usize + 1, false)
        } else if i < -(self.left.len() as isize) {
            self.left.reserve(1024);
            self.left.grow(-i as usize, false);
        }
    }

    fn set(&mut self, i: isize, val: bool) {
        if i >= 0 {
            self.right.set(i as usize, val);
        } else {
            self.left.set((-i - 1) as usize, val);
        }
    }

    fn count(&self) -> usize {
        self.left.iter().filter(|x| *x).count() + self.right.iter().filter(|x| *x).count()
    }
}

impl Index<isize> for Tape {
    type Output = bool;

    fn index(&self, i: isize) -> &bool {
        if i >= 0 {
            &self.right[i as usize]
        } else {
            &self.left[(-i - 1) as usize]
        }
    }
}

fn parse_transition(lines: &mut Lines) -> (usize, Direction, char) {
    let next_value: usize = lines
        .next()
        .unwrap()
        .strip_prefix("    - Write the value ")
        .unwrap()
        .strip_suffix(".")
        .unwrap()
        .parse()
        .unwrap();

    let direction = match lines
        .next()
        .unwrap()
        .strip_prefix("    - Move one slot to the ")
        .unwrap()
    {
        "left." => Direction::Left,
        "right." => Direction::Right,
        _ => panic!("invalid direction"),
    };

    let next_state = lines
        .next()
        .unwrap()
        .strip_prefix("    - Continue with state ")
        .unwrap()
        .chars()
        .next()
        .unwrap();

    (next_value, direction, next_state)
}

fn parse_rule(input: &str) -> (char, Vec<(usize, Direction, char)>) {
    let mut lines = input.lines();
    let mut rules = vec![];

    let state = lines
        .next()
        .unwrap()
        .strip_prefix("In state ")
        .unwrap()
        .chars()
        .next()
        .unwrap();

    //   If the current value is 0:
    lines.next().unwrap();
    rules.push(parse_transition(&mut lines));

    //   If the current value is 1:
    lines.next().unwrap();
    rules.push(parse_transition(&mut lines));

    (state, rules)
}

fn parse_blueprint(
    input: &str,
) -> IResult<&str, (char, u64, HashMap<char, Vec<(usize, Direction, char)>>)> {
    let mut groups = input.split("\n\n");

    let mut lines = groups.next().expect("invalid input").lines();

    let (_, start) =
        preceded(tag("Begin in state "), anychar)(lines.next().expect("no begin line"))?;

    let (_, steps) = preceded(tag("Perform a diagnostic checksum after "), u64)(
        lines.next().expect("no begin line"),
    )?;

    let states = groups.map(parse_rule).collect();

    Ok(("", (start, steps, states)))
}

pub fn solve(input: &str) -> Solution<usize, &str> {
    let (start, steps, states) = parse_blueprint(input).expect("invalid input").1;

    let mut tape = Tape::new();
    let mut state = start;
    let mut index = 0;

    tape.grow(0);

    for _i in 0..steps {
        let (next_value, direction, next_state) = states[&state][tape[index] as usize];

        tape.set(index, next_value != 0);
        state = next_state;

        index += match direction {
            Direction::Right => 1,
            Direction::Left => -1,
        };

        tape.grow(index);
    }

    Solution(tape.count(), "")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day25.txt")).0 == 3);
    }
}
