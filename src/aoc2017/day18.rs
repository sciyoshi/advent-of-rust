use crate::Solution;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i64, space0},
    combinator::map,
    sequence::{delimited, tuple},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

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
    Snd(Reg),
    Set(Reg, Arg),
    Add(Reg, Arg),
    Mul(Reg, Arg),
    Mod(Reg, Arg),
    Rcv(Reg),
    Jgz(Arg, Arg),
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

fn run(ins: &[Cmd], ident: i64, tx: Option<Sender<i64>>, rx: Option<Receiver<i64>>) -> i64 {
    let mut pc = 0;
    let mut regs = HashMap::<char, i64>::new();

    regs.insert('p', ident);

    let mut played = 0;
    let mut sent = 0;

    while let Some(cmd) = ins.get(pc as usize) {
        match cmd {
            Cmd::Snd(r) => {
                if let Some(sender) = &tx {
                    sender.send(*regs.get(r).unwrap_or(&0)).unwrap();
                    sent += 1;
                } else {
                    played = *regs.get(r).unwrap_or(&0);
                }
            }
            Cmd::Set(r, a) => *regs.entry(*r).or_insert(0) = a.value(&regs),
            Cmd::Add(r, a) => *regs.entry(*r).or_insert(0) += a.value(&regs),
            Cmd::Mul(r, a) => *regs.entry(*r).or_insert(0) *= a.value(&regs),
            Cmd::Mod(r, a) => *regs.entry(*r).or_insert(0) %= a.value(&regs),
            Cmd::Rcv(r) => {
                if let Some(receiver) = &rx {
                    if let Ok(val) = receiver.recv_timeout(Duration::from_secs(1)) {
                        *regs.entry(*r).or_insert(0) = val;
                    } else {
                        return sent;
                    }
                } else {
                    return played;
                }
            }
            Cmd::Jgz(r, a) => {
                if r.value(&regs) > 0 {
                    pc += a.value(&regs);
                    continue;
                }
            }
        }

        pc += 1;
    }

    return 0;
}

fn parse_inst(input: &str) -> IResult<&str, Cmd> {
    alt((
        map(tuple((tag("snd"), reg)), |(_, reg)| Cmd::Snd(reg)),
        map(tuple((tag("set"), reg, arg)), |(_, a, b)| Cmd::Set(a, b)),
        map(tuple((tag("add"), reg, arg)), |(_, a, b)| Cmd::Add(a, b)),
        map(tuple((tag("mul"), reg, arg)), |(_, a, b)| Cmd::Mul(a, b)),
        map(tuple((tag("mod"), reg, arg)), |(_, a, b)| Cmd::Mod(a, b)),
        map(tuple((tag("rcv"), reg)), |(_, reg)| Cmd::Rcv(reg)),
        map(tuple((tag("jgz"), arg, arg)), |(_, a, b)| Cmd::Jgz(a, b)),
    ))(input)
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let ins: Vec<Cmd> = input
        .lines()
        .map(|l| parse_inst(l).expect("invalid line").1)
        .collect();

    let part1 = run(&ins, 0, None, None);

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let ins = Arc::new(ins);
    let ins1 = Arc::clone(&ins);
    let ins2 = Arc::clone(&ins);

    let child1 = thread::spawn(move || {
        run(&ins1, 0, Some(tx1), Some(rx2));
    });

    let child2 = thread::spawn(move || run(&ins2, 1, Some(tx2), Some(rx1)));

    let _res1 = child1.join().unwrap();
    let res2 = child2.join().unwrap();

    Solution(part1, res2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day18.txt")).0 == 4);
    }
}
