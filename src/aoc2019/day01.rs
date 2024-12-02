use crate::{Solution, utils::extract_integers};

fn fuel(mut mass: usize) -> usize {
    let mut total = 0;
    while mass >= 9 {
        mass = mass / 3 - 2;
        total += mass;
    }
    total
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let modules: Vec<usize> = extract_integers(input);

    let part1 = modules.iter().map(|mass| mass / 3 - 2).sum();
    let part2 = modules.into_iter().map(fuel).sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("100756") == crate::Solution(33583, 50346));
    }
}
