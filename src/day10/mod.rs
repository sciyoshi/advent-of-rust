use std::io::{self, BufRead};

fn step(lengths: &[u8], rope: &mut Vec<usize>, rounds: usize) {
	let len = rope.len();

	let mut skip = 0;
	let mut pos = 0;

	for _round in 0..rounds {
		for &length in lengths {
			for i in 0..(length as usize / 2) {
				rope.swap((pos + i) % len, (pos + length as usize - i - 1) % len);
			}

			pos += length as usize + skip;
			skip += 1;
		}
	}
}

pub fn knothash<T: IntoIterator<Item=u8>>(line: T) -> u128 {
	let mut els: Vec<u8> = line.into_iter().collect();

	els.extend(&[17, 31, 73, 47, 23]);

	let mut rope: Vec<_> = (0..256).collect();

	step(&els, &mut rope, 64);

	let mut result: u128 = 0;

	for chunk in rope.chunks(16) {
		result <<= 8;
		result |= chunk.iter().fold(0, |acc, &v| acc ^ v as u8) as u128;
	}

	return result;
}

pub fn solve() {
	let stdin = io::stdin();
	let line = stdin.lock().lines().next().unwrap().unwrap();
	let lengths: Vec<_> = line.clone()
		.split(",")
		.filter_map(|el| el.parse::<u8>().ok())
		.collect();

	let mut rope: Vec<_> = (0..256).collect();

	step(&lengths, &mut rope, 1);

	println!("[Part 1] Product is: {}", rope[0] * rope[1]);

	println!("[Part 2] {:032x}", knothash(line.bytes()));
}
