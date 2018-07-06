use std::io::{self, BufRead};
use std::collections::HashMap;
use nom::*;
use crate::util::num;
use primal::is_prime;

type Reg = char;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Arg {
	Reg(Reg),
	Val(i64)
}

impl Arg {
	fn value(self, regs: &HashMap<Reg, i64>) -> i64 {
		match self {
			Arg::Reg(ref r) => *regs.get(r).unwrap_or(&0),
			Arg::Val(v) => v
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

named!(reg(&str) -> Reg, do_parse!(
	c: anychar >> (c)
));

named!(arg(&str) -> Arg, alt!(
	num => { |v: i64| Arg::Val(v) } |
	anychar => { |c| Arg::Reg(c) }
));

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

crate fn solve() {
	let stdin = io::stdin();
	let ins: Vec<Cmd> = stdin.lock().lines()
		.filter_map(|l| l.ok())
		.filter_map(|l| ws!(l.as_str(),
			alt!(
				do_parse!(tag_s!("set") >> a: reg >> b: arg >> (Cmd::Set(a, b))) |
				do_parse!(tag_s!("sub") >> a: reg >> b: arg >> (Cmd::Sub(a, b))) |
				do_parse!(tag_s!("mul") >> a: reg >> b: arg >> (Cmd::Mul(a, b))) |
				do_parse!(tag_s!("jnz") >> a: arg >> b: arg >> (Cmd::Jnz(a, b)))
			)
		).to_result().ok())
		.collect();

	println!("[Part 1] Mul ops: {}", run(&ins, true).1);

	let regs = run(&ins[..8], false).0;
	let min = regs[&'b'] as u64;
	let max = regs[&'c'] as u64;
	let step = if let Cmd::Sub(_reg, Arg::Val(step)) = ins[30] { -step as usize } else { 17 };
	let count = (min..=max).step_by(step).filter(|&n| !is_prime(n)).count();

	println!("[Part 2] Output value: {}", count);
}
