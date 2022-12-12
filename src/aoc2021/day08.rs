use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = input.lines().collect();

    let mut part1 = 0;
    let mut part2 = 0;

    for line in data {
        let all = line
            .split_whitespace()
            .filter(|&v| v != "|")
            .map(|v| v.chars().map(|c| 1 << (c as u8 - 'a' as u8)).sum::<u8>())
            .collect::<Vec<_>>();
        let digits = &all[..10];
        let output = &all[10..];

        let mut map = [0u8; 10];

        for d in digits {
            match d.count_ones() {
                2 => map[1] = *d,
                3 => map[7] = *d,
                4 => map[4] = *d,
                7 => map[8] = *d,
                _ => (),
            }
        }

        for d in digits {
            let c = [
                (map[1] & d).count_ones(),
                (map[4] & d).count_ones(),
                (map[7] & d).count_ones(),
                (map[8] & d).count_ones(),
            ];
            match c {
                [2, 3, 3, 6] => map[0] = *d,
                [1, 2, 2, 5] => map[2] = *d,
                [2, 3, 3, 5] => map[3] = *d,
                [1, 3, 2, 5] => map[5] = *d,
                [1, 3, 2, 6] => map[6] = *d,
                [2, 4, 3, 6] => map[9] = *d,
                _ => (),
            }
        }

        part1 += output
            .iter()
            .filter(|&d| [2, 3, 4, 7].contains(&d.count_ones()))
            .count();

        part2 += output
            .iter()
            .map(|&d| map.iter().position(|&v| v == d).unwrap())
            .reduce(|a, b| a * 10 + b)
            .unwrap();
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day08.txt")) == crate::Solution(26, 61229));
    }
}
