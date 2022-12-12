use crate::Solution;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap() as usize)
                .collect()
        })
        .collect();

    let width = data[0].len();

    let mut counts = vec![0; width];

    for line in &data {
        for (i, c) in line.iter().enumerate() {
            if *c == 1 {
                counts[i] += 1;
            }
        }
    }

    let mut v1 = 0;
    let mut v2 = 0;

    for i in 0..width {
        if counts[i] > data.len() / 2 {
            v1 += 1 << (width - 1 - i);
        } else {
            v2 += 1 << (width - 1 - i);
        }
    }

    let part1 = v1 * v2;

    v1 = 0;
    v2 = 0;

    let mut d1 = data.clone();
    let mut d2 = data.clone();

    for i in 0..width {
        let c1 = d1.iter().map(|l| l[i]).sum::<usize>();
        let c2 = d2.iter().map(|l| l[i]).sum::<usize>();

        let b1 = if d1.len() == 1 {
            c1
        } else {
            (c1 >= (d1.len() + 1) / 2) as usize
        };

        let b2 = if d2.len() == 1 {
            c2
        } else {
            (c2 < (d2.len() + 1) / 2) as usize
        };

        d1.retain(|l| l[i] == b1);
        d2.retain(|l| l[i] == b2);

        v1 += b1 * (1 << (width - 1 - i));
        v2 += b2 * (1 << (width - 1 - i));
    }

    let part2 = v1 * v2;

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010") == crate::Solution(198, 230));
    }
}
