use itertools::Itertools;
use std::io::{self, BufRead};

use crate::util::ints;

type Pt3 = [i64; 3];

trait Rotate3 {
    fn rotate(&self, n: usize) -> Self;
}

impl Rotate3 for Pt3 {
    fn rotate(&self, n: usize) -> Self {
        match n {
            0 => [self[0], self[1], self[2]],
            1 => [self[0], self[2], -self[1]],
            2 => [self[0], -self[2], self[1]],
            3 => [self[0], -self[1], -self[2]],
            4 => [self[1], self[0], -self[2]],
            5 => [self[1], self[2], self[0]],
            6 => [self[1], -self[2], -self[0]],
            7 => [self[1], -self[0], self[2]],
            8 => [self[2], self[0], self[1]],
            9 => [self[2], self[1], -self[0]],
            10 => [self[2], -self[1], self[0]],
            11 => [self[2], -self[0], -self[1]],
            12 => [-self[2], self[0], -self[1]],
            13 => [-self[2], self[1], self[0]],
            14 => [-self[2], -self[1], -self[0]],
            15 => [-self[2], -self[0], self[1]],
            16 => [-self[1], self[0], self[2]],
            17 => [-self[1], self[2], -self[0]],
            18 => [-self[1], -self[2], self[0]],
            19 => [-self[1], -self[0], -self[2]],
            20 => [-self[0], self[1], -self[2]],
            21 => [-self[0], self[2], self[1]],
            22 => [-self[0], -self[2], -self[1]],
            23 => [-self[0], -self[1], self[2]],
            _ => unreachable!(),
        }
    }
}

pub fn solve() {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let scanners: Vec<Vec<Pt3>> = data
        .split(|e| e == "")
        .map(|beacons| {
            beacons[1..]
                .iter()
                .map(|e| ints(e))
                .map(|v| [v[0], v[1], v[2]])
                .collect_vec()
        })
        .collect();

    println!("[Part 1] {:?}", scanners);
    println!("[Part 2] {:?}", 0);
}
