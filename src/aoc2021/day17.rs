use crate::util::ints;
use crate::Solution;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rect {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

/// The triangular root of a number.
fn triangular_root(n: i64) -> f64 {
    ((8. * n as f64 + 1.).sqrt() - 1.) / 2.
}

/// Given a target distance and the number of steps, returns the
/// initial velocity v such that sum_i=1^steps (v - i + 1) = dist.
fn steps_to_initial_vel(dist: i64, steps: i64) -> f64 {
    (steps as f64 - 1.) / 2. + (dist as f64) / (steps as f64)
}

/// Given a target distance and the number of steps after returning
/// to the origin, returns the initial velocity.
fn negative_steps_to_initial_vel(dist: i64, steps: i64) -> f64 {
    -(steps as f64 + 1.) / 2. - (dist as f64) / (steps as f64)
}

/// Given the minimum target y coordinate and the minimum number of
/// steps to reach it, returns the most steps (below the axis) that
/// could be taken.
fn max_steps_positive_vel(ymin: i64, min_steps: i64) -> f64 {
    (-2. * ymin as f64) / min_steps as f64
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Sweep {
    x: i64,
    open: bool,
    y1: i64,
    y2: i64,
}

fn sweep_area(rects: &[Rect]) -> i128 {
    let sweep = rects
        .iter()
        .flat_map(|r| {
            vec![
                Sweep {
                    x: r.x1,
                    y1: r.y1,
                    y2: r.y2 + 1,
                    open: true,
                },
                Sweep {
                    x: r.x2 + 1,
                    y1: r.y1,
                    y2: r.y2 + 1,
                    open: false,
                },
            ]
        })
        .sorted()
        .group_by(|el| el.x);

    let mut area: i128 = 0;
    let mut x = 0;
    let mut range: BTreeMap<i64, isize> = BTreeMap::new();

    for (key, group) in &sweep {
        if !range.is_empty() {
            let mut y = None;
            let mut count = 0;
            for (&yv, &n) in &range {
                count += n;
                if n > 0 && y.is_none() {
                    y = Some(yv);
                } else if n < 0 && count == 0 {
                    area += ((key - x) * (yv - y.unwrap())) as i128;
                    y = None;
                }
            }
        }

        x = key;

        for s in group {
            if s.open {
                range.entry(s.y1).and_modify(|n| *n += 1).or_insert(1);
                range.entry(s.y2).and_modify(|n| *n -= 1).or_insert(-1);
            } else {
                range.entry(s.y2).and_modify(|n| *n += 1).or_insert(1);
                range.entry(s.y1).and_modify(|n| *n -= 1).or_insert(-1);
            }
        }
    }

    area
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = ints(
        io::stdin()
            .lock()
            .lines()
            .flatten()
            .next()
            .unwrap()
            .as_str(),
    );

    let (xmin, xmax) = (data[0], data[1]);
    let (ymin, ymax) = (data[2], data[3]);

    // Works for some very large inputs!
    // let (xmin, xmax) = (135123456, 155123456);
    // let (ymin, ymax) = (-102123456, -78123456);

    println!("[Part 1] {:?}", ymin * (ymin + 1) / 2);

    let vxmin = triangular_root(xmin).ceil() as i64;
    let vxmax = triangular_root(xmax).floor() as i64;

    let mut rects: Vec<Rect> = vec![];

    for i in 1..=vxmax {
        let bounds = Rect {
            x1: if i > vxmin {
                vxmin
            } else {
                steps_to_initial_vel(xmin, i).ceil() as i64
            },
            y1: steps_to_initial_vel(ymin, i).ceil() as i64,
            x2: steps_to_initial_vel(xmax, i).floor() as i64,
            y2: steps_to_initial_vel(ymax, i).floor() as i64,
        };

        if let Some(last) = rects.last_mut() {
            if last.x1 == bounds.x1
                && last.x2 == bounds.x2
                && last.y1 <= bounds.y2
                && bounds.y1 <= last.y2
            {
                last.y1 = last.y1.min(bounds.y1);
                last.y2 = last.y2.max(bounds.y2);
            } else {
                rects.push(bounds);
            }
        } else {
            rects.push(bounds);
        }
    }

    let max_positive = max_steps_positive_vel(ymin, vxmax).ceil() as i64;

    for i in (1..=max_positive).rev() {
        let bounds = Rect {
            x1: vxmin,
            y1: (negative_steps_to_initial_vel(ymax, i).ceil() as i64)
                .max(steps_to_initial_vel(ymax, vxmin).ceil() as i64),
            x2: vxmax,
            y2: negative_steps_to_initial_vel(ymin, i).floor() as i64,
        };

        if let Some(last) = rects.last_mut() {
            if last.x1 == bounds.x1
                && last.x2 == bounds.x2
                && last.y1 <= bounds.y2
                && bounds.y1 <= last.y2
            {
                last.y1 = last.y1.min(bounds.y1);
                last.y2 = last.y2.max(bounds.y2);
            } else {
                rects.push(bounds);
            }
        } else {
            rects.push(bounds);
        }
    }

    for &r in &rects {
        println!("{:?}", r);
    }

    println!("[Part 2] {:?}", sweep_area(&rects));
}
