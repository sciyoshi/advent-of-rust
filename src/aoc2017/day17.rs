use std::io::{self, BufRead};

crate fn solve() {
	let stdin = io::stdin();
	let step: usize = stdin.lock().lines()
		.next().unwrap().unwrap()
		.parse().unwrap();

	let mut buf = vec![0];
	let mut pos = 0;

	for i in 1..2018 {
		pos = (pos + step) % buf.len() + 1;
		buf.insert(pos, i);
	}

	println!("[Part 1] Value is: {}", buf[pos + 1]);

	let mut buflen = 1;
	let mut pos = 0;
	let mut result = 0;

	for i in 1..50_000_000 {
		pos = (pos + step) % buflen + 1;
		buflen += 1;
		if pos == 1 {
			result = i;
		}
	}

	println!("[Part 2] Value is: {}", result);
}
