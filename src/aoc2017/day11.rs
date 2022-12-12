use crate::Solution;
use std::cmp;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Pt(i32, i32);

impl Add for Pt {
    type Output = Self;

    fn add(self, other: Pt) -> Pt {
        Pt(self.0 + other.0, self.1 + other.1)
    }
}

fn dist(pt: Pt) -> i32 {
    (pt.0.abs() + pt.1.abs() + (pt.0 + pt.1).abs()) / 2
}

pub fn solve(input: &str) -> Solution<i32, i32> {
    let (end, max) = input
        .trim()
        .split(",")
        .fold((Pt(0, 0), 0), |(pos, max), dir| {
            let next = pos
                + match dir {
                    "n" => Pt(0, 1),
                    "ne" => Pt(1, 0),
                    "se" => Pt(1, -1),
                    "s" => Pt(0, -1),
                    "sw" => Pt(-1, 0),
                    "nw" => Pt(-1, 1),
                    _ => panic!(),
                };

            (next, cmp::max(dist(next), max))
        });

    Solution(dist(end), max)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("se,sw,se,sw,sw").0 == 3);
    }
}
