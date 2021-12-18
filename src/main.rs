#![feature(generators, destructuring_assignment)]

use clap::{App, Arg};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

fn main() {
    let matches = App::new("Advent of Rust 2021")
        .author("Samuel Cormier-Iijima <samuel@cormier-iijima.com>")
        .arg(
            Arg::with_name("day")
                .required(true)
                .help("Day of the advent calendar")
                .validator(|str| {
                    str.parse::<u32>()
                        .or(Err("day must be an integer".to_owned()))
                        .and_then(|v| match v {
                            1..=25 => Ok(()),
                            _ => Err("day must be between 1 and 25".to_owned()),
                        })
                }),
        )
        .get_matches();

    match matches.value_of("day").unwrap().parse::<u32>().unwrap() {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        9 => day9::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        13 => day13::solve(),
        14 => day14::solve(),
        15 => day15::solve(),
        16 => day16::solve(),
        _ => (),
    }
}
