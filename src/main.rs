#![feature(portable_simd, array_chunks)]

use clap::Parser;

mod aoc2022;

#[derive(Parser)]
struct Args {
    year: u16,
    day: u16,
}

fn main() {
    let args = Args::parse();

    let (part1, part2) = match (args.year, args.day) {
        (2022, 01) => aoc2022::day1::solve(),
        (2022, 02) => aoc2022::day2::solve(),
        (2022, 03) => aoc2022::day3::solve(),
        (_, _) => panic!("invalid year/day"),
    };

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
