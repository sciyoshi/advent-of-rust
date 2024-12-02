use crate::Solution;
use crate::utils::extract_integers;

pub fn solve(input: &str) -> Solution<u32, u32> {
    let mut big: [u32; 3] = [0, 0, 0];

    for gr in input.split("\n\n") {
        let val = extract_integers::<u32>(gr).into_iter().sum();

        if val <= big[2] {
        } else if val <= big[1] {
            big[2] = val;
        } else if val <= big[0] {
            big[2] = big[1];
            big[1] = val;
        } else {
            big[2] = big[1];
            big[1] = big[0];
            big[0] = val;
        }
    }

    Solution(big[0], big[0] + big[1] + big[2])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day1.txt")) == crate::Solution(24000, 45000));
    }
}
