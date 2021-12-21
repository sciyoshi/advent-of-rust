use itertools::{EitherOrBoth, Itertools};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::io::{self, BufRead};

use crate::util::ints;

type Pt3 = [i64; 3];
type Rot3 = [[i64; 3]; 3];

const ROTATIONS: [Rot3; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
];

fn add_pt(a: &Pt3, b: &Pt3) -> Pt3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn sub_pt(a: &Pt3, b: &Pt3) -> Pt3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn pt_norm1(a: &Pt3) -> i64 {
    a[0].abs() + a[1].abs() + a[2].abs()
}

trait Rotate3 {
    fn rotate(&self, r: Rot3) -> Self;
}

impl Rotate3 for Pt3 {
    fn rotate(&self, r: Rot3) -> Self {
        [
            r[0][0] * self[0] + r[0][1] * self[1] + r[0][2] * self[2],
            r[1][0] * self[0] + r[1][1] * self[1] + r[1][2] * self[2],
            r[2][0] * self[0] + r[2][1] * self[1] + r[2][2] * self[2],
        ]
    }
}

fn rotate_compose(r1: Rot3, r2: Rot3) -> Rot3 {
    [
        [
            r1[0][0] * r2[0][0] + r1[0][1] * r2[1][0] + r1[0][2] * r2[2][0],
            r1[0][0] * r2[0][1] + r1[0][1] * r2[1][1] + r1[0][2] * r2[2][1],
            r1[0][0] * r2[0][2] + r1[0][1] * r2[1][2] + r1[0][2] * r2[2][2],
        ],
        [
            r1[1][0] * r2[0][0] + r1[1][1] * r2[1][0] + r1[1][2] * r2[2][0],
            r1[1][0] * r2[0][1] + r1[1][1] * r2[1][1] + r1[1][2] * r2[2][1],
            r1[1][0] * r2[0][2] + r1[1][1] * r2[1][2] + r1[1][2] * r2[2][2],
        ],
        [
            r1[2][0] * r2[0][0] + r1[2][1] * r2[1][0] + r1[2][2] * r2[2][0],
            r1[2][0] * r2[0][1] + r1[2][1] * r2[1][1] + r1[2][2] * r2[2][1],
            r1[2][0] * r2[0][2] + r1[2][1] * r2[1][2] + r1[2][2] * r2[2][2],
        ],
    ]
}

fn intersect_size(a: &BTreeMap<Pt3, usize>, b: &BTreeMap<Pt3, usize>, rot: Rot3) -> usize {
    a.into_iter()
        .merge_join_by(
            b.into_iter().map(|el| (el.0.rotate(rot), el.1)).sorted(),
            |&i, &j| i.0.cmp(&j.0),
        )
        .filter_map(|el| match el {
            EitherOrBoth::Both(i, j) => Some(i.1.min(j.1)),
            _ => None,
        })
        .sum()
}

fn diffs(pts: &[Pt3]) -> BTreeMap<Pt3, usize> {
    let mut result = BTreeMap::new();

    for v in pts.iter().permutations(2) {
        *result.entry(sub_pt(v[0], v[1])).or_insert(0) += 1;
    }

    result
}

struct Scanner {
    beacons: Vec<Pt3>,
    position: Option<Pt3>,
    discovered: bool,
    rotation: Rot3,
    diffs: BTreeMap<Pt3, usize>,
}

pub fn solve() {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let mut scanners: Vec<Scanner> = data
        .split(|e| e == "")
        .map(|beacons| {
            beacons[1..]
                .iter()
                .map(|e| ints(e))
                .map(|v| [v[0], v[1], v[2]])
                .collect_vec()
        })
        .map(|beacons| Scanner {
            diffs: diffs(&beacons),
            beacons,
            discovered: false,
            position: None,
            rotation: ROTATIONS[0],
        })
        .collect();

    let mut queue: VecDeque<usize> = VecDeque::new();

    queue.push_back(0);

    scanners[0].position = Some([0, 0, 0]);
    scanners[0].discovered = true;

    while let Some(i) = queue.pop_front() {
        for j in 0..scanners.len() {
            if i == j || scanners[j].discovered {
                continue;
            }

            for rot in ROTATIONS {
                if intersect_size(&scanners[i].diffs, &scanners[j].diffs, rot) >= 132 {
                    queue.push_back(j);
                    scanners[j].discovered = true;
                    scanners[j].rotation = rotate_compose(scanners[i].rotation, rot);
                    scanners[j].beacons = scanners[j]
                        .beacons
                        .iter()
                        .map(|b| b.rotate(scanners[j].rotation))
                        .collect();

                    let mut offsets = BTreeMap::new();

                    for (b1, b2) in scanners[i]
                        .beacons
                        .iter()
                        .cartesian_product(scanners[j].beacons.iter())
                    {
                        *offsets.entry(sub_pt(b1, b2)).or_insert(0) += 1;
                    }

                    let offset = offsets
                        .iter()
                        .max_by_key(|&(_, &v)| v)
                        .map(|(k, _)| k)
                        .unwrap();

                    scanners[j].position = Some(add_pt(&scanners[i].position.unwrap(), offset));

                    break;
                }
            }
        }
    }

    let mut beacons = BTreeSet::new();

    for scanner in &scanners {
        for beacon in &scanner.beacons {
            beacons.insert(add_pt(&scanner.position.unwrap(), &beacon));
        }
    }

    println!("[Part 1] {:?}", beacons.len());

    println!(
        "[Part 2] {:?}",
        &scanners
            .iter()
            .combinations(2)
            .map(|pair| pt_norm1(&sub_pt(
                &pair[0].position.unwrap(),
                &pair[1].position.unwrap()
            )))
            .max()
            .unwrap()
    );
}
