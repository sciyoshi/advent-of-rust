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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut input = String::new();

    stdin().lock().read_to_string(&mut input)?;

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
        (2017, 12) => println!("{}", aoc2017::day12::solve(&input)),
        (2017, 13) => println!("{}", aoc2017::day13::solve(&input)),
        (2017, 14) => println!("{}", aoc2017::day14::solve(&input)),
        (2017, 15) => println!("{}", aoc2017::day15::solve(&input)),
        (2017, 16) => println!("{}", aoc2017::day16::solve(&input)),
        (2017, 17) => println!("{}", aoc2017::day17::solve(&input)),
        (2017, 18) => println!("{}", aoc2017::day18::solve(&input)),
        (2017, 19) => println!("{}", aoc2017::day19::solve(&input)),
        (2017, 20) => println!("{}", aoc2017::day20::solve(&input)),
        // (2017, 21) => println!("{}", aoc2017::day21::solve(&input)),
        // (2017, 22) => println!("{}", aoc2017::day22::solve(&input)),
        // (2017, 23) => println!("{}", aoc2017::day23::solve(&input)),
        // (2017, 24) => println!("{}", aoc2017::day24::solve(&input)),
        // (2017, 25) => println!("{}", aoc2017::day25::solve(&input)),
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
        (2022, 11) => println!("{}", aoc2022::day11::solve(&input)),
        (_, _) => panic!("invalid year/day"),
    };

    Ok(())
}
