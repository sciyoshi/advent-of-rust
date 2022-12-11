use crate::util::num;
use crate::Solution;
use nom::*;
use std::collections::HashMap;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Point3D(i64, i64, i64);

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Self) -> Self::Output {
        Point3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Particle {
    pos: Point3D,
    vel: Point3D,
    acc: Point3D,
}

impl Particle {
    fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

named!(point3d(&str) -> Point3D, do_parse!(
    tag_s!("<") >>
    x: call!(num) >>
    tag_s!(",") >>
    y: call!(num) >>
    tag_s!(",") >>
    z: call!(num) >>
    tag_s!(">") >>
    (Point3D(x as i64, y as i64, z as i64))
));

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();
    let mut particles: Vec<(usize, Particle)> = vec![];

    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();

        let particle = do_parse!(
            line.as_str(),
            tag_s!("p=")
                >> p: point3d
                >> tag_s!(", v=")
                >> v: point3d
                >> tag_s!(", a=")
                >> a: point3d
                >> (Particle {
                    pos: p,
                    vel: v,
                    acc: a
                })
        )
        .unwrap()
        .1;

        particles.push((i, particle));
    }

    let mut colliding = particles.clone();

    for _ in 0..5000 {
        for (_, p) in particles.iter_mut() {
            p.update();
        }
    }

    let closest = particles
        .iter()
        .min_by_key(|(_, p)| p.pos.0.abs() + p.pos.1.abs() + p.pos.2.abs())
        .unwrap();

    println!("[Part 1] Closest particle: {}", closest.0);

    for _ in 0..5000 {
        let mut positions = HashMap::new();

        for (_, p) in colliding.iter_mut() {
            p.update();

            let count = positions.entry(p.pos).or_insert(0);
            *count += 1;
        }

        colliding.retain(|(_, p)| positions[&p.pos] == 1);
    }

    println!("[Part 2] Particles remaining: {}", colliding.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
