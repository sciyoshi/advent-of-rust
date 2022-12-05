use std::io::stdin;

pub fn solve() {
    let mut big: [u32; 3] = [0, 0, 0];
    let mut cur = 0;

    for line in stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if cur <= big[2] {
            } else if cur <= big[1] {
                big[2] = cur;
            } else if cur <= big[0] {
                big[2] = big[1];
                big[1] = cur;
            } else {
                big[2] = big[1];
                big[1] = big[0];
                big[0] = cur;
            }

            cur = 0;
        } else {
            cur += line.parse::<u32>().unwrap();
        }
    }

    println!("part1: {}", big[0]);
    println!("part2: {}", big[0] + big[1] + big[2]);
}
