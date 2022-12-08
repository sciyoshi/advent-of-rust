use crate::Solution;
use itertools::Itertools;

fn all_unique<const N: usize>(chars: &[char; N]) -> bool {
    chars.iter().all_unique()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let chars = input.chars().collect::<Vec<_>>();

    let part1 = chars.array_windows::<4>().position(all_unique).unwrap() + 4;
    let part2 = chars.array_windows::<14>().position(all_unique).unwrap() + 14;

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == crate::Solution(7, 19));
        assert!(super::solve("bvwbjplbgvbhsrlpgdmjqwftvncz") == crate::Solution(5, 23));
        assert!(super::solve("nppdvjthqldpwncqszvftbrmjlhg") == crate::Solution(6, 23));
        assert!(super::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == crate::Solution(10, 29));
        assert!(super::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == crate::Solution(11, 26));
    }
}
