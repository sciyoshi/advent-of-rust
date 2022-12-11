use crate::Solution;

struct Gen {
    value: u64,
    factor: u64,
    check: u64,
}

impl Iterator for Gen {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.value = (self.value * self.factor) % 2147483647;

            if self.value % self.check == 0 {
                return Some(self.value);
            }
        }
    }
}

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();

    let vals: Vec<u64> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.split_whitespace().last().unwrap().to_string())
        .filter_map(|el| el.parse().ok())
        .collect();

    let gen1 = Gen {
        value: vals[0],
        factor: 16807,
        check: 1,
    };
    let gen2 = Gen {
        value: vals[1],
        factor: 48271,
        check: 1,
    };

    let part1 = gen1
        .zip(gen2)
        .take(40_000_000)
        .filter(|&(v1, v2)| v1 as u16 == v2 as u16)
        .count();

    println!("[Part 1] Count is: {}", part1);

    let gen1 = Gen {
        value: vals[0],
        factor: 16807,
        check: 4,
    };
    let gen2 = Gen {
        value: vals[1],
        factor: 48271,
        check: 8,
    };

    let part2 = gen1
        .zip(gen2)
        .take(5_000_000)
        .filter(|&(v1, v2)| v1 as u16 == v2 as u16)
        .count();

    println!("[Part 2] Count is: {}", part2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
