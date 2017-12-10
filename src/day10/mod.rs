use std::io::{self, BufRead};

fn step(lengths: &[usize], rope: &mut Vec<usize>, rounds: usize) {
	let len = rope.len();

	let mut skip = 0;
	let mut pos = 0;

	for _round in 0..rounds {
		for length in lengths {
			for i in 0..(length / 2) {
				rope.swap((pos + i) % len, (pos + length - i - 1) % len);
			}

			pos += length + skip;
			skip += 1;
		}
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let line = stdin.lock().lines().next().unwrap().unwrap();
	let lengths: Vec<_> = line.clone()
		.split(",")
		.filter_map(|el| el.parse::<usize>().ok())
		.collect();

	let mut rope: Vec<_> = (0..256).collect();

	step(&lengths, &mut rope, 1);

	println!("[Part 1] Product is: {}", rope[0] * rope[1]);

	let mut lengths: Vec<usize> = line.bytes().map(|el| el as usize).collect();

	lengths.extend(&[17, 31, 73, 47, 23]);

	let mut rope: Vec<_> = (0..256).collect();

	step(&lengths, &mut rope, 64);

	let dense: Vec<String> = rope.chunks(16)
		.map(|chunk| chunk.iter().fold(0, |acc, &v| acc ^ v as u8))
		.map(|v| format!("{:x}", v))
		.collect();

	println!("[Part 2] {}", dense.join(""));
}
