use crate::util::Pt;
use crate::Solution;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Default for Status {
    fn default() -> Self {
        Status::Clean
    }
}

fn run(mut map: HashMap<Pt, Status>, steps: u32, evolved: bool) -> u32 {
    let mut pos = Pt(0, 0);
    let mut dir = Pt::n();
    let mut count = 0;

    for _i in 0..steps {
        let entry = map.entry(pos).or_default();

        match entry {
            Status::Clean => {
                dir = dir.rot90l();
                if evolved {
                    *entry = Status::Weakened;
                } else {
                    *entry = Status::Infected;
                    count += 1;
                }
            }
            Status::Weakened => {
                *entry = Status::Infected;
                count += 1;
            }
            Status::Infected => {
                dir = dir.rot90r();
                *entry = if evolved {
                    Status::Flagged
                } else {
                    Status::Clean
                };
            }
            Status::Flagged => {
                dir = -dir;
                *entry = Status::Clean;
            }
        }

        pos = pos + dir;
    }

    count
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();

    let mut map: HashMap<Pt, Status> = HashMap::new();

    let lines: Vec<String> = stdin.lock().lines().filter_map(|l| l.ok()).collect();
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;

    for (line, j) in lines.into_iter().zip((-(height / 2)..=height / 2).rev()) {
        for (c, i) in line.chars().zip(-(width / 2)..=width / 2) {
            map.insert(
                Pt(i, j),
                if c == '#' {
                    Status::Infected
                } else {
                    Status::Clean
                },
            );
        }
    }

    println!(
        "[Part 1] Times infected: {}",
        run(map.clone(), 10_000, false)
    );
    println!("[Part 2] Times infected: {}", run(map, 10_000_000, true));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
