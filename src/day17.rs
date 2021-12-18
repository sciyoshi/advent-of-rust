use std::io::{self, BufRead};

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let (xmin, xmax) = (117, 164);
    let (ymin, ymax) = (-140, -89);

    println!("[Part 1] {:?}", ymin * (ymin + 1) / 2);

    println!("[Part 2] {:?}", 0);
}
