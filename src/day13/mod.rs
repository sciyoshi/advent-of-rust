use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn solve() {
	let stdin = io::stdin();

	let mut heights = HashMap::<u32, u32>::new();

	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let split: Vec<_> = line.split(": ").collect();

		heights.insert(split[0].parse().unwrap(), split[1].parse().unwrap());
	}

	let severity: u32 = heights.iter()
		.filter(|&(&pos, &height)| pos % (2 * (height - 1)) == 0)
		.map(|(pos, height)| pos * height)
		.sum();

	println!("[Part 1] Severity is: {}", severity);

	let wait: u32 = (0..)
		.filter(|wait| !heights.iter()
			.any(|(&pos, &height)| (wait + pos) % (2 * (height - 1)) == 0))
		.next()
		.unwrap();

	println!("[Part 2] Wait time is: {}", wait);
}
