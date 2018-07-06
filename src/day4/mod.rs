use std::io::{self, BufRead};
use itertools::Itertools;

crate fn solve() {
	// Read stdin into an array of vectors of words
	let stdin = io::stdin();
	let lines: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(|w| w.to_string()).collect())
		.collect();

	// Count lines where all words are unique
	let count1 = lines.iter()
		.filter(|line| line.iter().unique().count() == line.len())
		.count();

	println!("[Part 1] Valid passphrases: {}", count1);

	// Count lines where all sorted words are unique (to detect anagrams)
	let count1 = lines.iter()
		.filter(|line| {
			let words: Vec<_> = line.iter().map(|w| w.chars().sorted()).collect();
			words.iter().unique().count() == words.len()
		})
		.count();

	println!("[Part 2] Valid passphrases: {}", count1);
}