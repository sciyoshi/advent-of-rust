use crate::utils::Pt;
use crate::Solution;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution<i32, i32> {
    let input: u32 = input.parse().unwrap();

    // Create a grid mapping points to their contents
    let mut grid = HashMap::new();

    // Start at 0,0 facing south (we'll turn left right away)
    let mut pos = Pt(0, 0);
    let mut dir = Pt::s();

    for i in 1..input {
        // Add the current point to the grid
        grid.insert(pos, i);

        // Turn left if we can
        if !grid.contains_key(&(pos + dir.rot90l())) {
            dir = dir.rot90l();
        }

        // Advance in the current direction
        pos = pos + dir;
    }

    let part1 = pos.norm1();

    // Create a grid mapping points to their contents
    let mut grid = HashMap::new();

    // Add the first value at 0,0
    grid.insert(Pt(0, 0), 1);

    // Start at 1,0 facing east (we'll turn left right away)
    let mut pos = Pt(1, 0);
    let mut dir = Pt::e();

    loop {
        // Add the sum of the neighboring point values to the grid
        let sum = pos.nb8().iter().map(|pt| grid.get(pt).unwrap_or(&0)).sum();

        grid.insert(pos, sum);

        if sum > input {
            return Solution(part1, sum as i32);
        }

        // Turn left if we can
        if !grid.contains_key(&(pos + dir.rot90l())) {
            dir = dir.rot90l();
        }

        // Advance in the current direction
        pos = pos + dir;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("1024").0 == 31);
        assert!(super::solve("800").1 == 806);
    }
}
