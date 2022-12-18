use itertools::Itertools;
use std::collections::BTreeSet;

use crate::{
    utils::{extract_integers, Pt},
    Solution,
};

struct Sensor {
    sensor: Pt<isize>,
    beacon: Pt<isize>,
    distance: isize,
}

fn ranges(data: &[Sensor], y: isize, max: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
    data.iter()
        .filter_map(move |v| {
            let d = v.distance - (v.sensor.1 - y).abs();

            if d > 0 && ((v.sensor.0 + d >= 0) || (v.sensor.0 - d <= max)) {
                Some((v.sensor.0 - d, v.sensor.0 + d))
            } else {
                None
            }
        })
        .sorted_by_key(|f| f.0)
}

fn all_overlap(mut ranges: impl Iterator<Item = (isize, isize)>) -> Option<isize> {
    let (_, mut max) = ranges.next().unwrap();

    for (next_min, next_max) in ranges {
        if next_min == max + 2 {
            return Some(max + 1);
        }
        max = max.max(next_max);
    }

    None
}

fn range_union_size(
    mut ranges: impl Iterator<Item = (isize, isize)>,
    beacons: &BTreeSet<isize>,
) -> isize {
    let (mut min, mut max) = ranges.next().unwrap();
    let mut total = 0;

    for (next_min, next_max) in ranges {
        if next_min > max + 1 {
            total +=
                max - min + 1 - beacons.iter().filter(|&&b| min <= b && b <= max).count() as isize;
            min = next_min;
            max = next_max;
        } else {
            max = max.max(next_max);
        }
    }

    total += max - min + 1 - beacons.iter().filter(|&&b| min <= b && b <= max).count() as isize;

    total
}

fn solve_size(input: &str, size: isize) -> Solution<isize, isize> {
    let mut data: Vec<Sensor> = input
        .lines()
        .map(|l| {
            let row = extract_integers(l);
            let sensor = Pt(row[0], row[1]);
            let beacon = Pt(row[2], row[3]);

            Sensor {
                sensor,
                beacon,
                distance: (beacon - sensor).norm1(),
            }
        })
        .collect();

    data.sort_by_key(|v| v.sensor.0);

    let mid = size / 2;
    let max = size;

    let row_beacons: BTreeSet<isize> = data
        .iter()
        .filter(|s| s.beacon.1 == mid)
        .map(|s| s.beacon.0)
        .collect();

    let part1 = range_union_size(ranges(&data, mid, max), &row_beacons);

    for y in 0..max {
        if let Some(x) = all_overlap(ranges(&data, y, max)) {
            return Solution(part1, x * 4000000 + y);
        }
    }

    unreachable!();
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    solve_size(input, 4000000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve_size(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                20
            ) == crate::Solution(26, 56000011)
        );
    }
}
