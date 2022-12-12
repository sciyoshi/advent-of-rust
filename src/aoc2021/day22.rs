use crate::util::ints;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
}

impl Cuboid {
    fn new(coords: &[i64]) -> Self {
        Cuboid {
            x1: coords[0],
            x2: coords[1],
            y1: coords[2],
            y2: coords[3],
            z1: coords[4],
            z2: coords[5],
        }
    }

    fn initialization(&self) -> bool {
        self.x1 >= -50
            && self.x2 <= 50
            && self.y1 >= -50
            && self.y2 <= 50
            && self.z1 >= -50
            && self.z2 <= 50
    }

    fn volume(&self) -> i64 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);

        if x1 <= x2 && y1 <= y2 && z1 <= z2 {
            Some(Cuboid {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        } else {
            None
        }
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|l| {
            (
                l.split_whitespace().nth(0).unwrap() == "on",
                Cuboid::new(&ints(&l)),
            )
        })
        .collect();

    let mut cuboids: HashMap<Cuboid, i64> = HashMap::new();
    let mut part1 = None;

    for &(on, cuboid) in &data {
        if !cuboid.initialization() && part1.is_none() {
            part1 = Some(
                cuboids
                    .iter()
                    .map(|(&cuboid, &count)| cuboid.volume() * count)
                    .sum::<i64>(),
            );

            println!("[Part 1] {:?}", part1.unwrap());
        }

        let mut updates: HashMap<Cuboid, i64> = HashMap::new();

        for (&other, &count) in cuboids.iter() {
            if let Some(intersect) = other.intersect(&cuboid) {
                let entry = updates.entry(intersect).or_insert(0);

                *entry -= count;
            }
        }

        if on {
            *cuboids.entry(cuboid).or_insert(0) += 1;
        }

        for (&other, &count) in updates.iter() {
            *cuboids.entry(other).or_insert(0) += count;
        }
    }

    println!(
        "[Part 2] {:?}",
        cuboids
            .iter()
            .map(|(&cuboid, &count)| cuboid.volume() * count)
            .sum::<i64>()
    );
}
