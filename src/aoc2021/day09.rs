use crate::Solution;
use std::collections::HashSet;

fn flood(grid: &[Vec<u32>], pt: (usize, usize)) -> usize {
    let mut stack = vec![pt];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    while let Some(p) = stack.pop() {
        seen.insert(p);

        if p.0 > 0
            && !seen.contains(&(p.0 - 1, p.1))
            && grid[p.0][p.1] < grid[p.0 - 1][p.1]
            && grid[p.0 - 1][p.1] < 9
        {
            stack.push((p.0 - 1, p.1));
        }

        if p.1 > 0
            && !seen.contains(&(p.0, p.1 - 1))
            && grid[p.0][p.1] < grid[p.0][p.1 - 1]
            && grid[p.0][p.1 - 1] < 9
        {
            stack.push((p.0, p.1 - 1));
        }

        if p.0 < grid.len() - 1
            && !seen.contains(&(p.0 + 1, p.1))
            && grid[p.0][p.1] < grid[p.0 + 1][p.1]
            && grid[p.0 + 1][p.1] < 9
        {
            stack.push((p.0 + 1, p.1));
        }

        if p.1 < grid[0].len() - 1
            && !seen.contains(&(p.0, p.1 + 1))
            && grid[p.0][p.1] < grid[p.0][p.1 + 1]
            && grid[p.0][p.1 + 1] < 9
        {
            stack.push((p.0, p.1 + 1));
        }
    }

    seen.len()
}

pub fn solve(input: &str) -> Solution<u32, usize> {
    let data: Vec<_> = input.lines().collect();

    let grid: Vec<Vec<u32>> = data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut part1 = 0;
    let mut basins = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            if i > 0 && grid[i - 1][j] <= col
                || j > 0 && grid[i][j - 1] <= col
                || i < grid.len() - 1 && grid[i + 1][j] <= col
                || j < row.len() - 1 && grid[i][j + 1] <= col
            {
                continue;
            }

            part1 += 1 + col;

            basins.push(flood(&grid, (i, j)));
        }
    }

    basins.sort();

    Solution(part1, basins[basins.len() - 3..].iter().product::<usize>())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("2199943210\n3987894921\n9856789892\n8767896789\n9899965678")
                == crate::Solution(15, 1134)
        );
    }
}
