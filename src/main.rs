#![feature(portable_simd, array_chunks, iter_array_chunks, type_alias_impl_trait)]

use clap::Parser;
use std::fmt::Display;
use std::io::{stdin, Read};

mod aoc2019;
mod aoc2022;
mod utils;

#[derive(PartialEq)]
pub struct Solution<P1: PartialEq + Display, P2: PartialEq + Display>(P1, P2);

impl<P1: PartialEq + Display, P2: PartialEq + Display> Display for Solution<P1, P2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[Part 1] {}\n[Part 2] {}", self.0, self.1))
    }
}

#[derive(Parser)]
struct Args {
    year: u16,
    day: u16,
}

fn main() {
    let args = Args::parse();

    let input = String::from_utf8(stdin().bytes().map(Result::unwrap).collect()).unwrap();

    match (args.year, args.day) {
        (2019, 02) => aoc2019::day2::solve(),
        (2019, 05) => aoc2019::day5::solve(),
        (2019, 06) => aoc2019::day6::solve(),
        (2022, 01) => println!("{}", aoc2022::day1::solve(&input)),
        (2022, 02) => println!("{}", aoc2022::day2::solve(&input)),
        (2022, 03) => println!("{}", aoc2022::day3::solve(&input)),
        (2022, 04) => println!("{}", aoc2022::day4::solve(&input)),
        (2022, 05) => println!("{}", aoc2022::day5::solve(&input)),
        (_, _) => panic!("invalid year/day"),
    };
}
