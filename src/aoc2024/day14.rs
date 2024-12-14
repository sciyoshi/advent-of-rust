use itertools::Itertools;

use crate::{
    Solution,
    util::euclid::{Box2, Pt2, Vec2, pt2},
    utils::extract_integers,
};

#[derive(Debug, Clone)]
struct Robot {
    pos: Pt2<isize>,
    vel: Vec2<isize>,
}

impl Robot {
    fn move_steps(&self, bounds: &Box2<isize>, steps: isize) -> Pt2<isize> {
        let x = (self.pos.x + self.vel.x * steps).rem_euclid(bounds.width());
        let y = (self.pos.y + self.vel.y * steps).rem_euclid(bounds.height());

        Pt2::new(x, y)
    }
}

fn part1(robots: &[Robot], width: isize, height: isize) -> usize {
    let bounds = Box2::new(pt2(0, 0), pt2(width, height));
    let mut quadrants = [0; 4];

    for robot in robots {
        match robot.move_steps(&bounds, 100).to_tuple() {
            (x, y) if x < bounds.width() / 2 && y < bounds.height() / 2 => quadrants[0] += 1,
            (x, y) if x > bounds.width() / 2 && y < bounds.height() / 2 => quadrants[1] += 1,
            (x, y) if x < bounds.width() / 2 && y > bounds.height() / 2 => quadrants[2] += 1,
            (x, y) if x > bounds.width() / 2 && y > bounds.height() / 2 => quadrants[3] += 1,
            _ => {}
        }
    }

    quadrants.iter().product()
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(extract_integers::<isize>)
        .map(|els| Robot {
            pos: Pt2::new(els[0], els[1]),
            vel: Vec2::new(els[2], els[3]),
        })
        .collect_vec()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let robots = parse_input(input);

    let p1 = part1(&robots, 101, 103);

    let bounds = Box2::new(pt2(0, 0), pt2(101, 103));

    for i in 0.. {
        let var = robots
            .iter()
            .map(|robot| robot.move_steps(&bounds, i))
            .map(|pos| (pos.x - bounds.width() / 2).pow(2) + (pos.y - bounds.height() / 2).pow(2))
            .sum::<isize>()
            / robots.len() as isize;

        if var < 800 {
            return Solution(p1, i as usize);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::part1(
                &super::parse_input(include_str!("examples/day14.txt")),
                11,
                7
            ) == 12
        );
    }
}
