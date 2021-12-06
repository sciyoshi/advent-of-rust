use std::io::{self, BufRead};

pub fn solve() {
    let data: Vec<Vec<usize>> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(2).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut counts = [0; 12];

    for line in &data {
        for (i, c) in line.iter().enumerate() {
            if *c == 1 {
                counts[i] += 1;
            }
        }
    }

    let mut v1 = 0;
    let mut v2 = 0;

    for i in 0..12 {
        if counts[i] > 500 {
            v1 += 1 << (11 - i);
        } else {
            v2 += 1 << (11 - i);
        }
    }

    println!("[Part 1] {:?}", v1 * v2);

    v1 = 0;
    v2 = 0;

    let mut d1 = data.clone();
    let mut d2 = data.clone();

    for i in 0..12 {
        let c1 = d1.iter().map(|l| l[i]).sum::<usize>();
        let c2 = d2.iter().map(|l| l[i]).sum::<usize>();

        let b1 = if d1.len() == 1 {
            c1
        } else {
            (c1 >= (d1.len() + 1) / 2) as usize
        };

        let b2 = if d2.len() == 1 {
            c2
        } else {
            (c2 < (d2.len() + 1) / 2) as usize
        };

        d1.retain(|l| l[i] == b1);
        d2.retain(|l| l[i] == b2);

        v1 += b1 * (1 << (11 - i));
        v2 += b2 * (1 << (11 - i));
    }

    println!("[Part 2] {:?}", v1 * v2);
}
