use crate::Solution;
use euclid::default::{Box2D, Point3D, Vector3D};
use euclid::point2;
use itertools::Itertools;
use ndarray::array;
use ndarray_linalg::Solve;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Hail {
    pos: Point3D<f64>,
    vel: Vector3D<f64>,
}

impl Hail {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(" @ ");
        let pos: Point3D<f64> = parts
            .next()
            .unwrap()
            .split(", ")
            .flat_map(str::parse)
            .collect_tuple::<(f64, f64, f64)>()
            .unwrap()
            .into();
        let vel = parts
            .next()
            .unwrap()
            .split(", ")
            .flat_map(str::parse)
            .collect_tuple::<(f64, f64, f64)>()
            .unwrap()
            .into();
        Self { pos, vel }
    }

    fn intersect(&self, other: &Self, area: &Box2D<f64>) -> bool {
        let t1 = (other.vel.x * (self.pos.y - other.pos.y)
            - other.vel.y * (self.pos.x - other.pos.x))
            / (self.vel.x * other.vel.y - self.vel.y * other.vel.x);

        let t2 = (self.vel.x * (self.pos.y - other.pos.y)
            - self.vel.y * (self.pos.x - other.pos.x))
            / (self.vel.x * other.vel.y - self.vel.y * other.vel.x);

        if t1 < 0f64 || t2 < 0f64 {
            return false;
        }

        let intersection = point2(self.pos.x + self.vel.x * t1, self.pos.y + self.vel.y * t1);

        area.contains(intersection)
    }
}

fn find_rock(hails: &[Hail]) -> f64 {
    let (h1, h2, h3) = (hails[0], hails[1], hails[2]);

    #[rustfmt::skip]
    let a = array![
        [0f64, h1.vel.z - h2.vel.z, h2.vel.y - h1.vel.y, 0f64, h2.pos.z - h1.pos.z, h1.pos.y - h2.pos.y],
        [h2.vel.z - h1.vel.z, 0f64, h1.vel.x - h2.vel.x, h1.pos.z - h2.pos.z, 0f64, h2.pos.x - h1.pos.x],
        [h1.vel.y - h2.vel.y, h2.vel.x - h1.vel.x, 0f64, h2.pos.y - h1.pos.y, h1.pos.x - h2.pos.x, 0f64],
        [0f64, h1.vel.z - h3.vel.z, h3.vel.y - h1.vel.y, 0f64, h3.pos.z - h1.pos.z, h1.pos.y - h3.pos.y],
        [h3.vel.z - h1.vel.z, 0f64, h1.vel.x - h3.vel.x, h1.pos.z - h3.pos.z, 0f64, h3.pos.x - h1.pos.x],
        [h1.vel.y - h3.vel.y, h3.vel.x - h1.vel.x, 0f64, h3.pos.y - h1.pos.y, h1.pos.x - h3.pos.x, 0f64],
    ];

    let b = array![
        h1.vel.z * h1.pos.y - h2.vel.z * h2.pos.y + h2.vel.y * h2.pos.z - h1.vel.y * h1.pos.z,
        h2.vel.z * h2.pos.x - h1.vel.z * h1.pos.x + h1.vel.x * h1.pos.z - h2.vel.x * h2.pos.z,
        h1.vel.y * h1.pos.x - h2.vel.y * h2.pos.x + h2.vel.x * h2.pos.y - h1.vel.x * h1.pos.y,
        h1.vel.z * h1.pos.y - h3.vel.z * h3.pos.y + h3.vel.y * h3.pos.z - h1.vel.y * h1.pos.z,
        h3.vel.z * h3.pos.x - h1.vel.z * h1.pos.x + h1.vel.x * h1.pos.z - h3.vel.x * h3.pos.z,
        h1.vel.y * h1.pos.x - h3.vel.y * h3.pos.x + h3.vel.x * h3.pos.y - h1.vel.x * h1.pos.y,
    ];

    let rock = a.solve(&b).unwrap();

    rock[0].round() + rock[1].round() + rock[2].round()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut hails = input.lines().map(Hail::parse).collect_vec();
    // let area = Box2D::new(point2(7f64, 7f64), point2(27f64, 27f64));
    let area = Box2D::new(
        point2(200_000_000_000_000f64, 200_000_000_000_000f64),
        point2(400_000_000_000_000f64, 400_000_000_000_000f64),
    );

    let count = hails
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a.intersect(b, &area))
        .count();

    // Due to precision issues, some combinations of hail will not be correct.
    // Sample 100 different combinations and find the mode
    let mut numbers_count = HashMap::new();
    for _ in 0..100 {
        hails.shuffle(&mut rand::thread_rng());
        let rock = find_rock(&hails) as usize;
        let count = numbers_count.entry(rock).or_insert(0);
        *count += 1;
    }

    let rock = numbers_count
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .0;

    Solution(count, rock)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day24.txt")) == crate::Solution(0, 47));
    }
}
