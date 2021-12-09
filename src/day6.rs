use crate::util::ints;
use std::io::{self, BufRead};

pub fn solve() {
    let data = ints(io::stdin().lock().lines().next().unwrap().unwrap().as_str());

    let mut counts = (0..=8)
        .map(|i| data.iter().filter(|&n| *n == i).count())
        .collect::<Vec<_>>();

    for i in 1..=256 {
        counts.rotate_left(1);
        counts[6] += counts[8];

        if i == 80 {
            println!("[Part 1] {:?}", counts.iter().sum::<usize>());
        }
    }

    println!("[Part 2] {:?}", counts.iter().sum::<usize>());
}
