use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve() {
	// Read stdin into an array of memory bank values
	let stdin = io::stdin();
	let mut data: Vec<_> = stdin.lock().lines()
		.next().unwrap().unwrap()
		.split_whitespace()
		.filter_map(|el| el.parse::<u32>().ok())
		.collect();

	let len = data.len();
	let mut count = 0;

	// Keep track of seen states, along with when we saw them
	let mut seen = HashMap::new();

	while !seen.contains_key(&data) {
		// Mark this state as seen
		seen.insert(data.clone(), count);

		// Find the first largest element (using the negative index to break ties)
		if let Some((i, &val)) = data.iter().enumerate()
			.max_by_key(|&(i, val)| (val, -(i as isize))) {
			// Remove the blocks from that bank
			data[i] = 0;

			// Redistribute, starting with the next index
			for j in 0..(val as usize) {
				data[(i + j + 1) % len] += 1;
			}
		}

		count += 1;
	}

	println!("[Part 1] Cycles: {}", count);
	println!("[Part 2] Cycles: {}", count - seen.get(&data).unwrap());
}