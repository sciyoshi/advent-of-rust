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

    #[cfg(test)]
    fn example() -> Self {
        let mut seams = Self::new(4);

        seams.add(Pt2::new(8, 0), Vec2::e(), Pt2::new(3, 4), Vec2::w());
        seams.add(Pt2::new(8, 3), Vec2::s(), Pt2::new(7, 4), Vec2::w());
        seams.add(Pt2::new(12, 0), Vec2::e(), Pt2::new(0, 4), Vec2::n());
        seams.add(Pt2::new(3, 7), Vec2::w(), Pt2::new(8, 11), Vec2::e());
        seams.add(Pt2::new(7, 7), Vec2::w(), Pt2::new(8, 8), Vec2::n());
        seams.add(Pt2::new(11, 4), Vec2::n(), Pt2::new(12, 3), Vec2::e());
        seams.add(Pt2::new(11, 8), Vec2::n(), Pt2::new(15, 3), Vec2::s());

        seams
    }

    fn input() -> Self {
        let mut seams = Self::new(50);

        seams.add(Pt2::new(50, 50), Vec2::e(), Pt2::new(49, 49), Vec2::s());
        seams.add(Pt2::new(99, 50), Vec2::n(), Pt2::new(149, 199), Vec2::s());
        seams.add(Pt2::new(99, 100), Vec2::n(), Pt2::new(149, 150), Vec2::w());
        seams.add(Pt2::new(0, 0), Vec2::e(), Pt2::new(100, 199), Vec2::e());
        seams.add(Pt2::new(0, 49), Vec2::s(), Pt2::new(50, 199), Vec2::e());
        seams.add(Pt2::new(0, 99), Vec2::s(), Pt2::new(50, 150), Vec2::n());
        seams.add(Pt2::new(49, 99), Vec2::w(), Pt2::new(50, 100), Vec2::n());

        seams
    }
}

#[derive(Debug)]
struct Board {
    walls: HashSet<Pt2<isize>>,
    range_x: Vec<(isize, isize)>,
    range_y: Vec<(isize, isize)>,
}

impl Board {
    fn parse(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut range_x = vec![];
        let mut range_y = vec![];

        for (y, line) in input.lines().rev().enumerate() {
            range_x.push((isize::MAX, isize::MIN));

            for (x, c) in line.chars().enumerate() {
                if range_y.len() <= x {
                    range_y.push((isize::MAX, isize::MIN));
                }

                if c == '#' {
                    walls.insert(Pt2::new(x as isize, y as isize));
                }

                if c != ' ' {
                    range_x[y] = (range_x[y].0.min(x as isize), range_x[y].1.max(x as isize));
                    range_y[x] = (range_y[x].0.min(y as isize), range_y[x].1.max(y as isize));
                }
            }
        }

        Board {
            walls,
            range_x,
            range_y,
        }
    }

    fn start_point(&self) -> Pt2<isize> {
        let y = self.range_x.len() - 1;
        Pt2::new(self.range_x[y].0, y as isize)
    }

    fn step(
        &self,
        pt: Pt2<isize>,
        dir: Vec2<isize>,
        seams: Option<&Seams>,
    ) -> (Pt2<isize>, Vec2<isize>) {
        if let Some(seams) = seams {
            if let Some(&(new_pt, new_dir)) = seams.map.get(&(pt, dir)) {
                (new_pt, new_dir)
            } else {
                (pt + dir, dir)
            }
        } else {
            let mut new_pt = pt + dir;

            let range = if dir.x == 0 {
                self.range_y[pt.x as usize]
            } else {
                self.range_x[pt.y as usize]
            };

            if dir.x == 0 {
                if new_pt.y < range.0 {
                    new_pt.y = range.1;
                } else if new_pt.y > range.1 {
                    new_pt.y = range.0;
                }
            } else {
                if new_pt.x < range.0 {
                    new_pt.x = range.1;
                } else if new_pt.x > range.1 {
                    new_pt.x = range.0;
                }
            }

            (new_pt, dir)
        }
    }

    fn walk(
        &self,
        mut pt: Pt2<isize>,
        mut dir: Vec2<isize>,
        steps: usize,
        seams: Option<&Seams>,
    ) -> (Pt2<isize>, Vec2<isize>) {
        for _ in 0..steps {
            let (new_pt, new_dir) = self.step(pt, dir, seams);

            if self.walls.contains(&new_pt) {
                return (pt, dir);
            }

            pt = new_pt;
            dir = new_dir;
        }

        (pt, dir)
    }

    fn password(&self, moves: &[Move], seams: Option<&Seams>) -> isize {
        let mut pt = self.start_point();
        let mut dir = Vec2::new(1, 0);

        for step in moves {
            match step {
                &Move::Forward(n) => {
                    (pt, dir) = self.walk(pt, dir, n, seams);
                }
                &Move::TurnLeft => {
                    dir = dir.rot90l();
                }
                &Move::TurnRight => {
                    dir = dir.rot90r();
                }
            }
        }

        1000 * (self.range_x.len() as isize - pt.y)
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

    let board = Board::parse(parts.next().unwrap());
    let steps = parse_steps(parts.next().unwrap()).unwrap().1;

    let part1 = board.password(&steps, None);
    let part2 = board.password(&steps, Some(&Seams::input()));

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::{parse_steps, Board, Seams};

    #[test]
    fn test_example() {
        let input = include_str!("examples/day22.txt");
        let mut parts = input.split("\n\n");

        let board = Board::parse(parts.next().unwrap());
        let steps = parse_steps(parts.next().unwrap()).unwrap().1;

        assert!(board.password(&steps, None) == 6032);
        assert!(board.password(&steps, Some(&Seams::example())) == 5031);
    }
}
