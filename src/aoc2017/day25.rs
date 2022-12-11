use crate::util::num;
use bit_vec::BitVec;
use nom::*;
use std::collections::HashMap;
use std::io::{self, Read};
use std::ops::Index;

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

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_to_string(&mut input).unwrap();

    let (start, steps, states) = ws!(
        input.as_str(),
        do_parse!(
            tag_s!("Begin in state")
                >> start: terminated!(anychar, tag_s!("."))
                >> tag_s!("Perform a diagnostic checksum after")
                >> steps: terminated!(num, tag_s!("steps."))
                >> states:
                    many0!(ws!(do_parse!(
                        tag_s!("In state")
                            >> state: terminated!(anychar, tag_s!(":"))
                            >> transitions:
                                many1!(ws!(do_parse!(
                                    tag_s!("If the current value is")
                                        >> _value: terminated!(num, tag_s!(":"))
                                        >> tag_s!("- Write the value")
                                        >> next_value: terminated!(num, tag_s!("."))
                                        >> tag_s!("- Move one slot to the")
                                        >> direction:
                                            terminated!(
                                                alt!(
                                                    tag_s!("left") => { |_| Direction::Left } |
                                                    tag_s!("right") => { |_| Direction::Right }
                                                ),
                                                tag_s!(".")
                                            )
                                        >> tag_s!("- Continue with state")
                                        >> next_state: terminated!(anychar, tag_s!("."))
                                        >> ((next_value as usize, direction, next_state))
                                )))
                            >> ((state, transitions))
                    )))
                >> ((
                    start,
                    steps,
                    states
                        .into_iter()
                        .collect::<HashMap<char, Vec<(usize, Direction, char)>>>()
                ))
        )
    )
    .to_result()
    .unwrap();

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

    println!("[Part 1] Diagnostic checksum is: {}", tape.count());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
