use crate::Solution;

pub fn solve(input: &str) -> Solution<u32, u32> {
    // Convert each character to a digit in base 10
    let vals: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    // Pair up digits using zip, and a cycled iterator skipped by 1
    let captcha1: u32 = vals
        .iter()
        .zip(vals.iter().cycle().skip(1))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();

    // Ditto here, but skip by half the number of digits
    let captcha2: u32 = vals
        .iter()
        .zip(vals.iter().cycle().skip(vals.len() / 2))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();

    Solution(captcha1, captcha2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("1122").0 == 3);
        assert!(super::solve("1111").0 == 4);
        assert!(super::solve("1234").0 == 0);
        assert!(super::solve("91212129").0 == 9);
        assert!(super::solve("1212").1 == 6);
        assert!(super::solve("1221").1 == 0);
        assert!(super::solve("1234225").1 == 4);
        assert!(super::solve("123123").1 == 12);
        assert!(super::solve("12131415").1 == 4);
    }
}
