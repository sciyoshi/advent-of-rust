use crate::Solution;

pub fn solve(input: &str) -> Solution<i64, i64> {
    let step: usize = input.parse().unwrap();

    let mut buf = vec![0];
    let mut pos = 0;

    for i in 1..2018 {
        pos = (pos + step) % buf.len() + 1;
        buf.insert(pos, i);
    }

    let part1 = buf[pos + 1];

    let mut buflen = 1;
    let mut pos = 0;
    let mut result = 0;

    for i in 1..50_000_000 {
        pos = (pos + step) % buflen + 1;
        buflen += 1;
        if pos == 1 {
            result = i;
        }
    }

    Solution(part1, result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("3").0 == 638);
    }
}
