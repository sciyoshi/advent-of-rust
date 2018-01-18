use std::collections::HashMap;
use std::io::{self, BufRead};
use std::cmp;
use nom::alpha;
use util::num;

pub fn solve() {
	let stdin = io::stdin();
	let mut regs = HashMap::<String, i64>::new();
	let mut maxseen = 0;

	for line in stdin.lock().lines() {
		let line = line.unwrap();

		ws!(line.as_str(), do_parse!(
			reg1: alpha >>
			op1: alt_complete!(
				tag_s!("inc") |
				tag_s!("dec")) >>
			val1: num >>
			tag_s!("if") >>
			reg2: alpha >>
			op2: alt_complete!(
				tag_s!("<=") |
				tag_s!(">=") |
				tag_s!("<") |
				tag_s!(">") |
				tag_s!("==") |
				tag_s!("!=")) >>
			val2: num >> ({
				let reg2 = *regs.get(reg2).unwrap_or(&0);

				let cond = match op2 {
					"<=" => reg2 <= val2,
					">=" => reg2 >= val2,
					"<" => reg2 < val2,
					">" => reg2 > val2,
					"==" => reg2 == val2,
					"!=" => reg2 != val2,
					_ => panic!("unknown operator")
				};

				if cond {
					let reg1 = regs.entry(reg1.to_string()).or_insert(0);

					match op1 {
						"inc" => *reg1 += val1,
						"dec" => *reg1 -= val1,
						_ => panic!("unknown operator")
					}

					maxseen = cmp::max(maxseen, *reg1);
				}
			})
		));
	}

	println!("[Part 1] Largest value after is: {}", regs.values().cloned().max().unwrap());
	println!("[Part 2] Largest value during is: {}", maxseen);
}
