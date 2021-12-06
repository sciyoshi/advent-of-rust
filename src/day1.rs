use std::io::{self, BufRead};

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let (result, _) = data.iter().fold((0, i32::MAX), |(acc, y), &x| {
        (if x > y { acc + 1 } else { acc }, x)
    });

    println!("[Part 1] {:?}", result);

    let (result, _) = data.windows(3).fold((0, i32::MAX), |(acc, y), x| {
        let t = x[0] + x[1] + x[2];
        (if t > y { acc + 1 } else { acc }, t)
    });

    println!("[Part 2] {:?}", result);
}
