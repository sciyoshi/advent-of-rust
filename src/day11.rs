use crate::util::Pt;
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut grid: Vec<Vec<u32>> = data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let dim = (grid.len() as isize, grid[0].len() as isize);

    let mut part1 = 0;
    let mut i = 0;

    loop {
        i += 1;

        let mut flashes = vec![];

        for i in 0..dim.0 {
            for j in 0..dim.1 {
                grid[i as usize][j as usize] += 1;
                if grid[i as usize][j as usize] > 9 {
                    flashes.push(Pt(i, j));
                }
            }
        }

        let mut flashed = HashSet::new();

        while let Some(octopus) = flashes.pop() {
            if flashed.contains(&octopus) {
                continue;
            }

            flashed.insert(octopus);

            for nb in octopus.nb8() {
                if nb.within(0, 0, dim.0 - 1, dim.1 - 1) && !flashed.contains(&nb) {
                    grid[nb.0 as usize][nb.1 as usize] += 1;
                    if grid[nb.0 as usize][nb.1 as usize] > 9 {
                        flashes.push(nb);
                    }
                }
            }
        }

        for pt in &flashed {
            grid[pt.0 as usize][pt.1 as usize] = 0;
        }

        if i < 100 {
            part1 += flashed.len();
        }

        if flashed.len() as isize == dim.0 * dim.1 {
            break;
        }
    }

    println!("[Part 1] {:?}", part1);
    println!("[Part 2] {:?}", i);
}
