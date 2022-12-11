#![feature(
    box_syntax,
    anonymous_lifetime_in_impl_trait,
    portable_simd,
    array_chunks,
    array_windows,
    iter_array_chunks,
    type_alias_impl_trait
)]

use clap::Parser;
use std::fmt::Display;
use std::io::{stdin, Read};

mod aoc2017;
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
        (2017, 01) => println!("{}", aoc2017::day01::solve(&input)),
        (2017, 02) => println!("{}", aoc2017::day02::solve(&input)),
        (2017, 03) => println!("{}", aoc2017::day03::solve(&input)),
        (2017, 04) => println!("{}", aoc2017::day04::solve(&input)),
        (2017, 05) => println!("{}", aoc2017::day05::solve(&input)),
        (2017, 06) => println!("{}", aoc2017::day06::solve(&input)),
        (2017, 07) => println!("{}", aoc2017::day07::solve(&input)),
        (2017, 08) => println!("{}", aoc2017::day08::solve(&input)),
        (2017, 09) => println!("{}", aoc2017::day09::solve(&input)),
        (2017, 10) => println!("{}", aoc2017::day10::solve(&input)),
        (2017, 11) => println!("{}", aoc2017::day11::solve(&input)),
        (2019, 02) => aoc2019::day2::solve(),
        (2019, 05) => aoc2019::day5::solve(),
        (2019, 06) => aoc2019::day6::solve(),
        (2022, 01) => println!("{}", aoc2022::day01::solve(&input)),
        (2022, 02) => println!("{}", aoc2022::day02::solve(&input)),
        (2022, 03) => println!("{}", aoc2022::day03::solve(&input)),
        (2022, 04) => println!("{}", aoc2022::day04::solve(&input)),
        (2022, 05) => println!("{}", aoc2022::day05::solve(&input)),
        (2022, 06) => println!("{}", aoc2022::day06::solve(&input)),
        (2022, 07) => println!("{}", aoc2022::day07::solve(&input)),
        (2022, 08) => println!("{}", aoc2022::day08::solve(&input)),
        (2022, 09) => println!("{}", aoc2022::day09::solve(&input)),
        (2022, 10) => println!("{}", aoc2022::day10::solve(&input)),
        (_, _) => panic!("invalid year/day"),
    };
}
