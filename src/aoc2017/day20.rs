use crate::Solution;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, space0},
    combinator::map,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};
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

fn point3d(input: &str) -> IResult<&str, Point3D> {
    map(
        delimited(
            tag("<"),
            separated_list1(delimited(space0, tag(","), space0), i64),
            tag(">"),
        ),
        |pts| Point3D(pts[0], pts[1], pts[2]),
    )(input)
}

fn parse_particle(input: &str) -> IResult<&str, Particle> {
    let (input, _) = tag("p=")(input)?;
    let (input, pos) = point3d(input)?;
    let (input, _) = tag(", v=")(input)?;
    let (input, vel) = point3d(input)?;
    let (input, _) = tag(", a=")(input)?;
    let (input, acc) = point3d(input)?;

    Ok((input, Particle { pos, vel, acc }))
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut particles: Vec<(usize, Particle)> = vec![];

    for (i, line) in input.lines().enumerate() {
        let particle = parse_particle(line).expect("invalid particle").1;

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

    for _ in 0..5000 {
        let mut positions = HashMap::new();

        for (_, p) in colliding.iter_mut() {
            p.update();

            let count = positions.entry(p.pos).or_insert(0);
            *count += 1;
        }

        colliding.retain(|(_, p)| positions[&p.pos] == 1);
    }

    Solution(closest.0, colliding.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>").0
                == 0
        );
        assert!(super::solve("p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\np=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\np=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\np=<3,0,0>, v=<-1,0,0>, a=<0,0,0>").1 == 1);
    }
}
