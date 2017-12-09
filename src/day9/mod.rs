use std::io::{self, BufRead};
use nom::IResult;

struct Stats {
	score: usize,
	garbage: usize,
}

named!(garbage(&[u8]) -> usize, delimited!(
	tag!("<"),
	fold_many0!(
		alt!(
			none_of!("!>") => { |_| 1 } |
			pair!(tag!("!"), take!(1)) => { |_| 0 }
		),
		0,
		|acc: usize, item: usize| acc + item
	),
	tag!(">")
));

fn group(input: &[u8], depth: usize) -> IResult<&[u8], Stats> {
	delimited!(input,
		tag!("{"),
		fold_many0!(
			alt!(
				apply!(group, depth + 1) |
				map!(garbage, |len| Stats { score: 0, garbage: len }) |
				value!(Stats { score: 0, garbage: 0 }, tag!(","))
			),
			Stats { score: depth + 1, garbage: 0 },
			|acc: Stats, item: Stats| Stats {
				score: acc.score + item.score,
				garbage: acc.garbage + item.garbage
			}
		),
		tag!("}")
	)
}

pub fn solve() {
	let stdin = io::stdin();
	let line = stdin.lock().lines().next().unwrap().unwrap();

	let (_, stats) = group(line.as_bytes(), 0).unwrap();

	println!("[Part 1] Group score is: {}", stats.score);
	println!("[Part 2] Garbage count is: {}", stats.garbage);
}
