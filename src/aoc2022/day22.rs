use std::collections::{HashMap, HashSet};

use crate::util::euclid::{Pt2, Vec2, Vec2Ext};
use crate::Solution;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u64, combinator::map, multi::many1,
    IResult,
};

#[derive(Debug)]
enum Move {
    Forward(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Default, Debug)]
struct Seams {
    size: isize,
    map: HashMap<(Pt2<isize>, Vec2<isize>), (Pt2<isize>, Vec2<isize>)>,
}

enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn offset(&self, clockwise: bool) -> Vec2<isize> {
        match (self, clockwise) {
            (Side::Bottom, false) | (Side::Left, true) => Vec2::new(0, 0),
            (Side::Right, false) | (Side::Bottom, true) => Vec2::new(1, 0),
            (Side::Top, false) | (Side::Right, true) => Vec2::new(1, 1),
            (Side::Left, false) | (Side::Top, true) => Vec2::new(0, 1),
        }
    }

    fn direction(&self, clockwise: bool) -> Vec2<isize> {
        match (self, clockwise) {
            (Side::Bottom, false) | (Side::Top, true) => Vec2::new(1, 0),
            (Side::Right, false) | (Side::Left, true) => Vec2::new(0, 1),
            (Side::Top, false) | (Side::Bottom, true) => Vec2::new(-1, 0),
            (Side::Left, false) | (Side::Right, true) => Vec2::new(0, -1),
        }
    }
}

impl Seams {
    fn new(size: isize) -> Self {
        Seams {
            size,
            map: HashMap::default(),
        }
    }

    fn add(&mut self, pt1: Pt2<isize>, d1: Vec2<isize>, pt2: Pt2<isize>, d2: Vec2<isize>) {
        for i in 0..self.size {
            self.map
                .insert((pt1 + d1 * i, d1.rot90r()), (pt2 + d2 * i, d2.rot90r()));
            self.map
                .insert((pt2 + d2 * i, d2.rot90l()), (pt1 + d1 * i, d1.rot90l()));
        }
    }

    fn add_cube(&mut self, p1: Pt2<isize>, side1: Side, p2: Pt2<isize>, side2: Side) {
        self.add(
            p1 * self.size + side1.offset(false) * (self.size - 1),
            side1.direction(false),
            p2 * self.size + side2.offset(true) * (self.size - 1),
            side2.direction(true),
        );
    }

    fn add_flat_x(&mut self, x1: isize, x2: isize, y: isize) {
        self.add(
            Pt2::new(x2 * self.size - 1, y * self.size),
            Vec2::n(),
            Pt2::new(x1 * self.size, y * self.size),
            Vec2::n(),
        )
    }

    fn add_flat_y(&mut self, y1: isize, y2: isize, x: isize) {
        self.add(
            Pt2::new(x * self.size, y1 * self.size),
            Vec2::e(),
            Pt2::new(x * self.size, y2 * self.size - 1),
            Vec2::e(),
        )
    }

    #[cfg(test)]
    fn example() -> Self {
        let mut seams = Self::new(4);

        //             ┌──2──┐
        //             2     3
        // ┌──2──┬──2──┼─────┤
        // 0     │     │     3
        // └──1──┴──1──┼─────┼──1──┐
        //             2     │     3
        //             └──0──┴──0──┘

        seams.add_flat_x(2, 3, 2);
        seams.add_flat_x(0, 3, 1);
        seams.add_flat_x(2, 4, 0);

        seams.add_flat_y(1, 2, 0);
        seams.add_flat_y(1, 2, 1);
        seams.add_flat_y(0, 3, 2);
        seams.add_flat_y(0, 1, 3);

        seams
    }

    #[cfg(test)]
    fn example_cube() -> Self {
        let mut seams = Self::new(4);

        //             ┌──D──┐
        //             E 2,2 g
        // ┌──d──┬──e──┼─────┤
        // C 0,1 │ 1,1 │ 2,1 f
        // └──A──┴──B──┼─────┼──F──┐
        //             b 2,0 │ 3,0 G
        //             └──a──┴──c──┘

        seams.add_cube(Pt2::new(2, 0), Side::Bottom, Pt2::new(0, 1), Side::Bottom);
        seams.add_cube(Pt2::new(2, 0), Side::Left, Pt2::new(1, 1), Side::Bottom);
        seams.add_cube(Pt2::new(3, 0), Side::Bottom, Pt2::new(0, 1), Side::Left);
        seams.add_cube(Pt2::new(0, 1), Side::Top, Pt2::new(2, 2), Side::Top);
        seams.add_cube(Pt2::new(1, 1), Side::Top, Pt2::new(2, 2), Side::Left);
        seams.add_cube(Pt2::new(2, 1), Side::Right, Pt2::new(3, 0), Side::Top);
        seams.add_cube(Pt2::new(2, 2), Side::Right, Pt2::new(3, 0), Side::Right);

        seams
    }

    fn input() -> Self {
        let mut seams = Self::new(50);

        //       ┌──4──┬──4──┐
        //       1     │     3
        //       ├─────┼──3──┘
        //       1     2
        // ┌──2──┼─────┤
        // 0     │     2
        // ├─────┼──1──┘
        // 0     1
        // └──0──┘

        seams.add_flat_x(1, 3, 3);
        seams.add_flat_x(1, 2, 2);
        seams.add_flat_x(0, 2, 1);
        seams.add_flat_x(0, 1, 0);

        seams.add_flat_y(0, 2, 0);
        seams.add_flat_y(1, 4, 1);
        seams.add_flat_y(3, 4, 2);

        seams
    }

    fn input_cube() -> Self {
        let mut seams = Self::new(50);

        //       ┌──E──┬──D──┐
        //       F 1,3 │ 2,3 B
        //       ├─────┼──C──┘
        //       G 1,2 c
        // ┌──g──┼─────┤
        // f 0,1 │ 1,1 b
        // ├─────┼──a──┘
        // e 0,0 A
        // └──d──┘

        seams.add_cube(Pt2::new(1, 1), Side::Bottom, Pt2::new(0, 0), Side::Right);
        seams.add_cube(Pt2::new(1, 1), Side::Right, Pt2::new(2, 3), Side::Right);
        seams.add_cube(Pt2::new(1, 2), Side::Right, Pt2::new(2, 3), Side::Bottom);
        seams.add_cube(Pt2::new(0, 0), Side::Bottom, Pt2::new(2, 3), Side::Top);
        seams.add_cube(Pt2::new(0, 0), Side::Left, Pt2::new(1, 3), Side::Top);
        seams.add_cube(Pt2::new(0, 1), Side::Left, Pt2::new(1, 3), Side::Left);
        seams.add_cube(Pt2::new(0, 1), Side::Top, Pt2::new(1, 2), Side::Left);

        seams
    }
}

struct Board {
    height: isize,
    start_x: isize,
    walls: HashSet<Pt2<isize>>,
    seams: Seams,
}

impl Board {
    fn parse(input: &str, start_x: isize, seams: Seams) -> Self {
        Board {
            height: input.lines().count() as isize,
            start_x,
            seams,
            walls: input
                .lines()
                .rev()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| Pt2::new(x as isize, y as isize))
                })
                .collect(),
        }
    }

    fn step(&self, pt: Pt2<isize>, dir: Vec2<isize>) -> (Pt2<isize>, Vec2<isize>) {
        if let Some(&(new_pt, new_dir)) = self.seams.map.get(&(pt, dir)) {
            (new_pt, new_dir)
        } else {
            (pt + dir, dir)
        }
    }

    fn walk(
        &self,
        mut pt: Pt2<isize>,
        mut dir: Vec2<isize>,
        steps: usize,
    ) -> (Pt2<isize>, Vec2<isize>) {
        for _ in 0..steps {
            let (new_pt, new_dir) = self.step(pt, dir);

            if self.walls.contains(&new_pt) {
                return (pt, dir);
            }

            pt = new_pt;
            dir = new_dir;
        }

        (pt, dir)
    }

    fn password(&self, moves: &[Move]) -> isize {
        let mut pt = Pt2::new(self.start_x, self.height - 1);
        let mut dir = Vec2::new(1, 0);

        for step in moves {
            match step {
                &Move::Forward(n) => {
                    (pt, dir) = self.walk(pt, dir, n);
                }
                &Move::TurnLeft => {
                    dir = dir.rot90l();
                }
                &Move::TurnRight => {
                    dir = dir.rot90r();
                }
            }
        }

        1000 * (self.height - pt.y)
            + 4 * (pt.x + 1)
            + match dir {
                Vec2 { x: 1, y: 0, .. } => 0,
                Vec2 { x: -1, y: 0, .. } => 2,
                Vec2 { x: 0, y: 1, .. } => 3,
                Vec2 { x: 0, y: -1, .. } => 1,
                _ => unreachable!(),
            }
    }
}

fn parse_steps(input: &str) -> IResult<&str, Vec<Move>> {
    many1(alt((
        map(u64, |v| Move::Forward(v as usize)),
        map(tag("L"), |_| Move::TurnLeft),
        map(tag("R"), |_| Move::TurnRight),
    )))(input)
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let mut parts = input.split("\n\n");
    let board = parts.next().unwrap();
    let steps = parse_steps(parts.next().unwrap()).unwrap().1;

    let part1 = Board::parse(board, 50, Seams::input()).password(&steps);
    let part2 = Board::parse(board, 50, Seams::input_cube()).password(&steps);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::{parse_steps, Board, Seams};

    #[test]
    fn test_example() {
        let input = include_str!("examples/day22.txt");
        let mut parts = input.split("\n\n");
        let board = parts.next().unwrap();
        let steps = parse_steps(parts.next().unwrap()).unwrap().1;

        assert!(Board::parse(board, 8, Seams::example()).password(&steps) == 6032);
        assert!(Board::parse(board, 8, Seams::example_cube()).password(&steps) == 5031);
    }
}
