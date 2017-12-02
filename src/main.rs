extern crate clap;
extern crate colored;
extern crate itertools;

use clap::{Arg, App};

mod day1;
mod day2;

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
						1...2 => Ok(()),
						_ => Err("day must be between 1 and 2".to_owned())
					})))
		.get_matches();

	match matches.value_of("day").unwrap().parse::<u32>().unwrap() {
		1 => day1::solve(),
		2 => day2::solve(),
		_ => ()
	}
}
