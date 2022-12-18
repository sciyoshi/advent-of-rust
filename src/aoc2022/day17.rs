use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, VecDeque},
    fmt::Display,
    hash::{Hash, Hasher},
};

use crate::{utils::Pt, Solution};

struct Shape {
    pts: &'static [Pt<isize>],
    width: isize,
    height: isize,
}

impl Shape {
    const fn new(pts: &'static [Pt<isize>]) -> Self {
        let mut i = 0;
        let mut width = 1;
        let mut height = 1;
        while i < pts.len() {
            if pts[i].0 + 1 > width {
                width = pts[i].0 + 1;
            }
            if pts[i].1 + 1 > height {
                height = pts[i].1 + 1;
            }
            i += 1;
        }
        Shape { pts, height, width }
    }
}

static SHAPES: &[Shape] = &[
    Shape::new(&[Pt(0, 0), Pt(1, 0), Pt(2, 0), Pt(3, 0)]),
    Shape::new(&[Pt(1, 0), Pt(0, 1), Pt(1, 1), Pt(2, 1), Pt(1, 2)]),
    Shape::new(&[Pt(0, 0), Pt(1, 0), Pt(2, 0), Pt(2, 1), Pt(2, 2)]),
    Shape::new(&[Pt(0, 0), Pt(0, 1), Pt(0, 2), Pt(0, 3)]),
    Shape::new(&[Pt(0, 0), Pt(0, 1), Pt(1, 0), Pt(1, 1)]),
];

const MEM: usize = 100;

struct Chamber {
    pts: BTreeSet<Pt<isize>>,
    wind: Vec<isize>,
    wind_pos: usize,
    height: isize,
    width: isize,
    shapes: usize,
    drops: VecDeque<Pt<isize>>,
    mem: BTreeMap<(usize, usize, u64), (usize, isize)>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in ((self.height - 100).max(0)..self.height).rev() {
            for col in 0..self.width {
                f.write_str(if self.pts.contains(&Pt(col, row)) {
                    "X"
                } else {
                    "."
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Chamber {
    fn from_wind(input: &str) -> Self {
        Chamber {
            pts: BTreeSet::new(),
            wind: input
                .chars()
                .filter_map(|c| match c {
                    '>' => Some(1),
                    '<' => Some(-1),
                    _ => None,
                })
                .collect(),
            wind_pos: 0,
            height: 0,
            width: 7,
            shapes: 0,
            drops: VecDeque::new(),
            mem: BTreeMap::new(),
        }
    }

    fn check(&self, shape: &Shape, pos: Pt<isize>) -> bool {
        pos.0 >= 0
            && pos.0 + shape.width <= self.width
            && pos.1 >= 0
            && !shape.pts.iter().any(|&pt| self.pts.contains(&(pt + pos)))
    }

    fn drop_shape(&mut self, shape: &Shape) -> Pt<isize> {
        let mut pos = Pt(2, self.height + 3);

        loop {
            let wind = self.wind[self.wind_pos];
            self.wind_pos = (self.wind_pos + 1) % self.wind.len();

            if self.check(shape, pos + Pt::e() * wind) {
                pos += Pt::e() * wind;
            }

            if self.check(shape, pos + Pt::s()) {
                pos += Pt::s();
            } else {
                break pos;
            }
        }
    }

    fn drop(&mut self) -> Option<isize> {
        let shape_pos = self.shapes % SHAPES.len();
        let shape = &SHAPES[shape_pos];
        self.shapes += 1;

        let pos = self.drop_shape(shape);
        let new_height = self.height.max(pos.1 + shape.height);

        if new_height != self.height {
            for drop in &mut self.drops {
                drop.1 += new_height - self.height;
            }
        }

        self.drops.push_back(Pt(pos.0, new_height - pos.1));
        if self.drops.len() > MEM {
            self.drops.pop_front();
        }

        let mut hasher = DefaultHasher::new();
        self.drops.hash(&mut hasher);
        let key = (shape_pos, self.wind_pos, hasher.finish());

        self.pts.extend(shape.pts.iter().map(|&pt| pt + pos));
        self.height = new_height;

        if let Some(&(last_shapes, last_height)) = self.mem.get(&key) {
            if (1_000_000_000_000 - self.shapes) % (self.shapes - last_shapes) == 0 {
                return Some(
                    self.height
                        + (1_000_000_000_000 - self.shapes as isize)
                            / (self.shapes as isize - last_shapes as isize)
                            * (self.height - last_height),
                );
            }
        }

        self.mem.insert(key, (self.shapes, new_height));

        None
    }
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let mut chamber = Chamber::from_wind(input);

    for _ in 0..2022 {
        chamber.drop();
    }

    let part1 = chamber.height;

    loop {
        if let Some(part2) = chamber.drop() {
            return Solution(part1, part2);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
                == crate::Solution(3068, 1514285714288)
        );
    }
}
