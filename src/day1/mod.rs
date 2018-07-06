use std::io::{self, BufRead};

crate fn solve() {
	// Get the first line from standard input
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap();

	// Convert each character to a digit in base 10
	let vals: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

	// Pair up digits using zip, and a cycled iterator skipped by 1
	let captcha1: u32 = vals.iter()
		.zip(vals.iter().cycle().skip(1))
		.filter_map(|(a, b)| if a == b { Some(a) } else { None })
		.sum();

	println!("[Part 1] CAPTCHA is: {}", captcha1.to_string());

	// Ditto here, but skip by half the number of digits
	let captcha2: u32 = vals.iter()
		.zip(vals.iter().cycle().skip(vals.len() / 2))
		.filter_map(|(a, b)| if a == b { Some(a) } else { None })
		.sum();

	println!("[Part 2] CAPTCHA is: {}", captcha2.to_string());
}
