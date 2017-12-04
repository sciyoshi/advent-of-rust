#![feature(const_fn, inclusive_range_syntax)]

extern crate num;
extern crate clap;
extern crate colored;
extern crate itertools;

use clap::{Arg, App};

mod util;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
	let matches = App::new("Advent of Rust 2017")
		.author("Samuel Cormier-Iijima <samuel@cormier-iijima.com>")
		.arg(Arg::with_name("day")
			.required(true)
			.help("Day of the advent calendar")
			.validator(|str|
				str.parse::<u32>()
					.or(Err("day must be an integer".to_owned()))
					.and_then(|v| match v {
						1...4 => Ok(()),
						_ => Err("day must be between 1 and 4".to_owned())
					})))
		.arg(Arg::with_name("value")
			.help("Problem input (for those with a single numeric input)")
			.required_if("day", "3")
			.validator(|str|
				str.parse::<u32>()
					.and(Ok(()))
					.or(Err("value must be an integer".to_owned()))))
		.get_matches();

	match matches.value_of("day").unwrap().parse::<u32>().unwrap() {
		1 => day1::solve(),
		2 => day2::solve(),
		3 => day3::solve(matches.value_of("value").unwrap().parse::<u32>().unwrap()),
		4 => day4::solve(),
		_ => ()
	}
}
