#![feature(portable_simd, array_chunks)]

use clap::Parser;

mod aoc2019;
mod aoc2022;
mod utils;

#[derive(Parser)]
struct Args {
    year: u16,
    day: u16,
}

fn main() {
    let args = Args::parse();

    match (args.year, args.day) {
        (2019, 06) => aoc2019::day6::solve(),
        (2022, 01) => aoc2022::day1::solve(),
        (2022, 02) => aoc2022::day2::solve(),
        (2022, 03) => aoc2022::day3::solve(),
        (2022, 04) => aoc2022::day4::solve(),
        (2022, 05) => aoc2022::day5::solve(),
        (_, _) => panic!("invalid year/day"),
    };
}
