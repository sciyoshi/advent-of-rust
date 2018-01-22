use std::io::{self, BufRead};
use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender, Receiver};
use nom::{digit, anychar};

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
	Snd(Reg),
	Set(Reg, Arg),
	Add(Reg, Arg),
	Mul(Reg, Arg),
	Mod(Reg, Arg),
	Rcv(Reg),
	Jgz(Arg, Arg),
}

named!(reg(&str) -> Reg, do_parse!(
	c: anychar >> (c)
));

named!(arg(&str) -> Arg, alt!(
	recognize!(pair!(opt!(tag_s!("-")), call!(digit))) => { |s: &str| Arg::Val(s.parse().unwrap()) } |
	anychar => { |c| Arg::Reg(c) }
));

enum Msg {
	Val(i64),
	Count(usize)
}

fn run(ins: &[Cmd], ident: i64, tx: Option<Sender<Msg>>, rx: Option<Receiver<Msg>>) -> i64 {
	let mut pc = 0;
	let mut regs = HashMap::<char, i64>::new();

	regs.insert('p', ident);

	let mut played = 0;
	let mut sent = 0;
	let mut received = 0;

	while let Some(cmd) = ins.get(pc as usize) {
		match cmd {
			Cmd::Snd(r) => {
				if let Some(sender) = &tx {
					sender.send(Msg::Val(*regs.get(r).unwrap_or(&0))).unwrap();
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
					if let Some(sender) = &tx {
						sender.send(Msg::Count(received)).unwrap_or(());
					}

					if let Ok(msg) = receiver.recv() {
						match msg {
							Msg::Val(val) => {
								received += 1;
								*regs.entry(*r).or_insert(0) = val;
							},
							Msg::Count(count) => {
								if count == sent {
									return sent as i64;
								}
								continue;
							}
						}
					} else {
						return sent as i64
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

pub fn solve() {
	let stdin = io::stdin();
	let ins: Vec<Cmd> = stdin.lock().lines()
		.filter_map(|l| l.ok())
		.filter_map(|l| ws!(l.as_str(),
			alt!(
				do_parse!(tag_s!("snd") >> reg: reg >> (Cmd::Snd(reg))) |
				do_parse!(tag_s!("set") >> a: reg >> b: arg >> (Cmd::Set(a, b))) |
				do_parse!(tag_s!("add") >> a: reg >> b: arg >> (Cmd::Add(a, b))) |
				do_parse!(tag_s!("mul") >> a: reg >> b: arg >> (Cmd::Mul(a, b))) |
				do_parse!(tag_s!("mod") >> a: reg >> b: arg >> (Cmd::Mod(a, b))) |
				do_parse!(tag_s!("rcv") >> reg: reg >> (Cmd::Rcv(reg))) |
				do_parse!(tag_s!("jgz") >> a: arg >> b: arg >> (Cmd::Jgz(a, b)))
			)
		).to_result().ok())
		.collect();

	println!("[Part 1] Recovered frequency is: {}", run(&ins, 0, None, None));

	let (tx1, rx1) = channel();
	let (tx2, rx2) = channel();

	let ins = Arc::new(ins);
	let ins1 = Arc::clone(&ins);
	let ins2 = Arc::clone(&ins);

	let child1 = thread::spawn(move || {
		run(&ins1, 0, Some(tx1), Some(rx2));
	});

	let child2 = thread::spawn(move || {
		run(&ins2, 1, Some(tx2), Some(rx1))
	});

	let _res1 = child1.join().unwrap();
	let res2 = child2.join().unwrap();

	println!("[Part 2] Sent: {}", res2);
}
