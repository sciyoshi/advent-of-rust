use std::{
    collections::HashSet,
    io::{self, BufRead},
};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let mut easts = HashSet::new();
    let mut souths = HashSet::new();

    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '>' => {
                    easts.insert((i, j));
                }
                'v' => {
                    souths.insert((i, j));
                }
                _ => (),
            };
        }
    }

    let mut steps = 0;
    let height = data.len();
    let width = data[0].len();

    loop {
        steps += 1;

        let mut east_moves = HashSet::new();

        for &cucumber in &easts {
            let pt = (cucumber.0, (cucumber.1 + 1) % width);
            if !souths.contains(&pt) && !easts.contains(&pt) {
                east_moves.insert(cucumber);
            }
        }

        for &cucumber in &east_moves {
            easts.remove(&cucumber);
            easts.insert((cucumber.0, (cucumber.1 + 1) % width));
        }

        let mut south_moves = HashSet::new();

        for &cucumber in &souths {
            let pt = ((cucumber.0 + 1) % height, cucumber.1);
            if !souths.contains(&pt) && !easts.contains(&pt) {
                south_moves.insert(cucumber);
            }
        }

        for &cucumber in &south_moves {
            souths.remove(&cucumber);
            souths.insert(((cucumber.0 + 1) % height, cucumber.1));
        }

        if south_moves.is_empty() && east_moves.is_empty() {
            break;
        }
    }

    println!("[Part 1] {:?}", steps);
}
