use crate::Solution;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i64, space0},
    combinator::map,
    sequence::{delimited, tuple},
};
use primal::is_prime;
use std::collections::HashMap;

type Reg = char;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Arg {
    Reg(Reg),
    Val(i64),
}

impl Arg {
    fn value(self, regs: &HashMap<Reg, i64>) -> i64 {
        match self {
            Arg::Reg(ref r) => *regs.get(r).unwrap_or(&0),
            Arg::Val(v) => v,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Cmd {
    Set(Reg, Arg),
    Sub(Reg, Arg),
    Mul(Reg, Arg),
    Jnz(Arg, Arg),
}

fn reg(input: &str) -> IResult<&str, char> {
    delimited(space0, anychar, space0)(input)
}

fn arg(input: &str) -> IResult<&str, Arg> {
    alt((
        map(delimited(space0, i64, space0), |v| Arg::Val(v)),
        map(reg, |c| Arg::Reg(c)),
    ))(input)
}

fn run(ins: &[Cmd], debug: bool) -> (HashMap<char, i64>, i64) {
    let mut pc = 0;
    let mut regs = HashMap::<char, i64>::new();
    let mut count = 0;

    if !debug {
        regs.insert('a', 1);
    }

    while let Some(cmd) = ins.get(pc as usize) {
        match cmd {
            Cmd::Set(r, a) => *regs.entry(*r).or_insert(0) = a.value(&regs),
            Cmd::Sub(r, a) => *regs.entry(*r).or_insert(0) -= a.value(&regs),
            Cmd::Mul(r, a) => {
                *regs.entry(*r).or_insert(0) *= a.value(&regs);
                count += 1;
            }
            Cmd::Jnz(r, a) => {
                if r.value(&regs) != 0 {
                    pc += a.value(&regs);
                    continue;
                }
            }
        }

        pc += 1;
    }

    (regs, count)
}

fn parse_inst(input: &str) -> IResult<&str, Cmd> {
    alt((
        map(tuple((tag("set"), reg, arg)), |(_, a, b)| Cmd::Set(a, b)),
        map(tuple((tag("sub"), reg, arg)), |(_, a, b)| Cmd::Sub(a, b)),
        map(tuple((tag("mul"), reg, arg)), |(_, a, b)| Cmd::Mul(a, b)),
        map(tuple((tag("jnz"), arg, arg)), |(_, a, b)| Cmd::Jnz(a, b)),
    ))(input)
}

pub fn solve(input: &str) -> Solution<i64, usize> {
    let ins: Vec<Cmd> = input
        .lines()
        .map(|l| parse_inst(l).expect("invalid instruction").1)
        .collect();

    let part1 = run(&ins, true).1;

    let regs = run(&ins[..8], false).0;
    let min = regs[&'b'] as u64;
    let max = regs[&'c'] as u64;
    let step = if let Cmd::Sub(_reg, Arg::Val(step)) = ins[30] {
        -step as usize
    } else {
        17
    };
    let count = (min..=max).step_by(step).filter(|&n| !is_prime(n)).count();

    Solution(part1, count)
}
