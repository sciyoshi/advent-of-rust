use std::collections::HashSet;

use crate::util::euclid::{pt2, Box2, Pt2, Pt2Ext, Vec2Ext};
use crate::Solution;
use pathfinding::directed::astar::astar;

#[derive(Default, Debug)]
struct Valley {
    width: isize,
    height: isize,
    blizzards_north: HashSet<Pt2<isize>>,
    blizzards_east: HashSet<Pt2<isize>>,
    blizzards_south: HashSet<Pt2<isize>>,
    blizzards_west: HashSet<Pt2<isize>>,
}

impl Valley {
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let mut valley = Valley::default();

        valley.height = lines.len() as isize - 2;
        valley.width = lines[0].len() as isize - 2;

        for (y, line) in lines[1..lines.len() - 1].iter().rev().enumerate() {
            for (x, c) in line.chars().skip(1).enumerate() {
                let pt = pt2(x as isize, y as isize);

                match c {
                    '^' => valley.blizzards_north.insert(pt),
                    '>' => valley.blizzards_east.insert(pt),
                    'v' => valley.blizzards_south.insert(pt),
                    '<' => valley.blizzards_west.insert(pt),
                    _ => true,
                };
            }
        }

        valley
    }

    fn pos_free(&self, time: isize, pos: Pt2<isize>) -> bool {
        pos == pt2(0, self.height)
            || pos == pt2(self.width - 1, -1)
            || Box2::new(pt2(0, 0), pt2(self.width, self.height)).contains(pos)
                && !self
                    .blizzards_north
                    .contains(&pt2(pos.x, (pos.y - time).rem_euclid(self.height)))
                && !self
                    .blizzards_east
                    .contains(&pt2((pos.x - time).rem_euclid(self.width), pos.y))
                && !self
                    .blizzards_south
                    .contains(&pt2(pos.x, (pos.y + time).rem_euclid(self.height)))
                && !self
                    .blizzards_west
                    .contains(&pt2((pos.x + time).rem_euclid(self.width), pos.y))
    }

    fn next_positions(&self, time: isize, pos: Pt2<isize>) -> Vec<((isize, Pt2<isize>), isize)> {
        pos.nb_ortho()
            .chain(std::iter::once(pos))
            .filter(|&pos| self.pos_free(time + 1, pos))
            .map(|pos| ((time + 1, pos), 1))
            .collect()
    }

    fn best_path(&self, start_time: isize, forward: bool) -> isize {
        let mut start = pt2(0, self.height);
        let mut end = pt2(self.width - 1, -1);

        if !forward {
            (start, end) = (end, start);
        }

        let (_, cost) = astar(
            &(start_time, start),
            |&(time, pos)| self.next_positions(time, pos),
            |&(_, pos)| (pos - end).norm1(),
            |&(_, pos)| pos == end,
        )
        .unwrap();

        start_time + cost
    }
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let valley = Valley::parse(input);

    let part1 = valley.best_path(0, true);
    let part2 = valley.best_path(valley.best_path(part1, false), true);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day24.txt")) == crate::Solution(18, 54));
    }
}
