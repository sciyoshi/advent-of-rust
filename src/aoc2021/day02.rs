use crate::Solution;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, space1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug, PartialEq)]
struct Pt {
    x: i64,
    y: i64,
}

fn parse(i: &str) -> IResult<&str, (Direction, i64)> {
    separated_pair(
        alt((
            map(tag("forward"), |_| Direction::Forward),
            map(tag("up"), |_| Direction::Up),
            map(tag("down"), |_| Direction::Down),
        )),
        space1,
        i64,
    )(i)
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let data: Vec<_> = input.lines().map(|line| parse(line).unwrap().1).collect();

    let mut pt = Pt { x: 0, y: 0 };

    for (dir, val) in &data {
        match dir {
            Direction::Up => pt.y += val,
            Direction::Down => pt.y -= val,
            Direction::Forward => pt.x += val,
        }
    }

    let part1 = pt.x * pt.y.abs();

    let mut pt = Pt { x: 0, y: 0 };
    let mut aim = 0;

    for (dir, val) in &data {
        match dir {
            Direction::Up => {
                aim += val;
            }
            Direction::Down => {
                aim -= val;
            }
            Direction::Forward => {
                pt.x += val;
                pt.y += val * aim;
            }
        }
    }

    let part2 = pt.x * pt.y.abs();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2")
                == crate::Solution(150, 900)
        );
    }
}
