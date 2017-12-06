use std::io::{self, BufRead};

fn run(mut data: Vec<isize>, updater: fn(isize) -> isize) -> usize {
	// Store a program counter (isize to allow negative)
	let mut pc = 0isize;
	let mut count = 0;

	while pc >= 0 && pc < data.len() as isize {
		let ins = data[pc as usize];
		data[pc as usize] += updater(ins);
		pc += ins;
		count += 1;
	}

	count
}

pub fn solve() {
	// Read stdin into an array of instructions
	let stdin = io::stdin();
	let data: Vec<_> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.filter_map(|el| el.parse::<isize>().ok())
		.collect();

	let count1 = run(data.clone(), |_ins| 1);

	println!("[Part 1] Count: {}", count1);

	let count2 = run(data.clone(), |ins| if ins >= 3 { -1 } else { 1 });

	println!("[Part 2] Count: {}", count2);
}