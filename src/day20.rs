use ndarray::{s, Array2};
use std::{
    io::{self, BufRead},
    ops::BitOrAssign,
};

pub fn solve() {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let alg = data[0].chars().map(|c| c == '#').collect::<Vec<bool>>();

    let pad = 50;
    let mut img: Array2<u16> = Array2::zeros((data.len() - 2 + 2 * pad, data[2].len() + 2 * pad));

    for (i, line) in data[2..].iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            img[(i + pad, j + pad)] = if c == '#' { 1 } else { 0 };
        }
    }

    for i in 0..50 {
        let mut next = img.clone();
        img <<= 4;
        img.slice_mut(s![..-1, ..-1])
            .bitor_assign(&next.slice(s![1.., 1..]));
        next <<= 1;
        img.slice_mut(s![..-1, ..])
            .bitor_assign(&next.slice(s![1.., ..]));
        next <<= 1;
        img.slice_mut(s![..-1, 1..])
            .bitor_assign(&next.slice(s![1.., ..-1]));
        next <<= 1;
        img.slice_mut(s![.., ..-1])
            .bitor_assign(&next.slice(s![.., 1..]));
        next <<= 2;
        img.slice_mut(s![.., 1..])
            .bitor_assign(&next.slice(s![.., ..-1]));
        next <<= 1;
        img.slice_mut(s![1.., ..-1])
            .bitor_assign(&next.slice(s![..-1, 1..]));
        next <<= 1;
        img.slice_mut(s![1.., ..])
            .bitor_assign(&next.slice(s![..-1, ..]));
        next <<= 1;
        img.slice_mut(s![1.., 1..])
            .bitor_assign(&next.slice(s![..-1, ..-1]));

        if i % 2 == 1 {
            img.slice_mut(s![0isize, ..]).bitor_assign(256 + 128 + 64);
            img.slice_mut(s![-1isize, ..]).bitor_assign(4 + 2 + 1);
            img.slice_mut(s![.., 0isize]).bitor_assign(256 + 32 + 4);
            img.slice_mut(s![.., -1isize]).bitor_assign(64 + 8 + 1);
        }

        img.mapv_inplace(|x| alg[x as usize] as u16);

        if i == 1 {
            println!("[Part 1] {:?}", img.sum());
        }
    }

    println!("[Part 2] {:?}", img.sum());
}
