#![feature(const_fn, inclusive_range_syntax, universal_impl_trait)]

#[macro_use]
extern crate nom;
extern crate num;
extern crate clap;
extern crate regex;
extern crate colored;
extern crate petgraph;
extern crate itertools;

use clap::{Arg, App};

mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

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
						1...11 => Ok(()),
						_ => Err("day must be between 1 and 11".to_owned())
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
		5 => day5::solve(),
		6 => day6::solve(),
		7 => day7::solve(),
		8 => day8::solve(),
		9 => day9::solve(),
		10 => day10::solve(),
		11 => day11::solve(),
		_ => ()
	}
}
