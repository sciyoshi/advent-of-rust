use crate::Solution;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, space1},
    combinator::{map, value},
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Copy, Clone, Debug)]
enum Inst {
    Addx(i64),
    Noop,
}

fn parse_inst(input: &str) -> IResult<&str, Inst> {
    alt((
        value(Inst::Noop, tag("noop")),
        map(separated_pair(tag("addx"), space1, i64), |(_, v)| {
            Inst::Addx(v)
        }),
    ))(input)
}

pub fn solve(input: &str) -> Solution<i64, String> {
    let insts = input
        .lines()
        .map(|l| parse_inst(l).finish().unwrap().1)
        .collect::<Vec<_>>();

    let mut cycle = 1;
    let mut x: i64 = 1;

    let checks = vec![20, 60, 100, 140, 180, 220];
    let mut strength = 0;
    let mut crt = vec![];

    for &inst in &insts {
        let start = x;
        let start_cycle = cycle;

        match inst {
            Inst::Noop => {
                crt.push(x.abs_diff(crt.len() as i64 % 40) <= 1);
                cycle += 1;
            }
            Inst::Addx(v) => {
                crt.push(x.abs_diff(crt.len() as i64 % 40) <= 1);
                crt.push(x.abs_diff(crt.len() as i64 % 40) <= 1);
                x += v;
                cycle += 2;
            }
        }

        if let Some(&c) = checks.iter().find(|&&c| start_cycle <= c && c < cycle) {
            strength += c * start;
        }
    }

    let part2 = "\n".to_string()
        + &crt
            .chunks(40)
            .map(|row| {
                row.iter()
                    .map(|&pixel| if pixel { '#' } else { '.' })
                    .collect::<String>()
            })
            .join("\n");

    Solution(strength, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day10.txt"))
                == crate::Solution(
                    13140,
                    "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                        .to_string()
                )
        );
    }
}
