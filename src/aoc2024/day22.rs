use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{Solution, utils::extract_integers};

struct Monkey {
    secret: usize,
}

impl Monkey {
    fn new(secret: usize) -> Self {
        Self { secret }
    }

    fn changes(&self) -> BTreeMap<[i8; 4], i8> {
        let mut result = BTreeMap::new();
        for (change, price) in self.iter().take(2001).map_windows(|&[a, b, c, d, e]| {
            let [a, b, c, d, e] = [
                (a % 10) as i8,
                (b % 10) as i8,
                (c % 10) as i8,
                (d % 10) as i8,
                (e % 10) as i8,
            ];
            ([b - a, c - b, d - c, e - d], e)
        }) {
            result.entry(change).or_insert(price);
        }
        result
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        let mut secret = self.secret;
        std::iter::from_fn(move || {
            let result = secret;
            secret = secret ^ ((secret << 6) % (1 << 24));
            secret = secret ^ ((secret >> 5) % (1 << 24));
            secret = secret ^ ((secret << 11) % (1 << 24));
            Some(result)
        })
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let monkeys = extract_integers(input)
        .iter()
        .map(|&secret| Monkey::new(secret))
        .collect_vec();

    let part1 = monkeys
        .iter()
        .map(|monkey| monkey.iter().nth(2000).unwrap())
        .sum();

    let changes = monkeys.iter().map(|monkey| monkey.changes()).collect_vec();

    let part2 = (0..4)
        .map(|_| -9..10)
        .multi_cartesian_product()
        .flat_map(|prod| {
            let mut price: usize = 0;
            for change in &changes {
                if let Some(&p) = change.get(&[prod[0], prod[1], prod[2], prod[3]]) {
                    price += p as usize;
                }
            }
            Some(price)
        })
        .max()
        .unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day22.txt")) == crate::Solution(37327623, 24));
    }
}
