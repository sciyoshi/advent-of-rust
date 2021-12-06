use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, space1},
    combinator::map,
    sequence::separated_pair,
    IResult,
};
use std::io::{self, BufRead};

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

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| parse(line.unwrap().as_str()).unwrap().1)
        .collect();

    let mut pt = Pt { x: 0, y: 0 };

    for (dir, val) in &data {
        match dir {
            Direction::Up => pt.y += val,
            Direction::Down => pt.y -= val,
            Direction::Forward => pt.x += val,
        }
    }

    println!("[Part 1] {:?}", pt.x * pt.y.abs());

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

    println!("[Part 2] {:?}", pt.x * pt.y.abs());
}
