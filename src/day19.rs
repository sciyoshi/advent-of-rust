use std::io::{self, BufRead};

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("[Part 1] {:?}", 0);
    println!("[Part 2] {:?}", 0);
}
