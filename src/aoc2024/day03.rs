use crate::Solution;
use regex::Regex;

pub fn solve(input: &str) -> Solution<usize, usize> {
    // find all cases of mul(x,y) with regex

    let part1 = Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<usize>().unwrap();
            let y = cap[2].parse::<usize>().unwrap();
            x * y
        })
        .sum();

    let mut part2 = 0;
    let mut enabled = true;

    Regex::new(r"mul\((\d+),(\d+)\)|(?P<do>do\(\))|(?P<dont>don't\(\))")
        .unwrap()
        .captures_iter(input)
        .for_each(|cap| {
            if cap.name("do").is_some() {
                enabled = true
            } else if cap.name("dont").is_some() {
                enabled = false
            } else if enabled {
                let x = cap[1].parse::<usize>().unwrap();
                let y = cap[2].parse::<usize>().unwrap();
                part2 += x * y;
            }
        });

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day03.txt")) == crate::Solution(161, 48));
    }
}
