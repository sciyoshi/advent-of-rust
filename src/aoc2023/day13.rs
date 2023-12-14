// Partially generated by ChatGPT
// https://chat.openai.com/share/1743e9b4-ca23-463d-9e4d-9bda335f1371

use crate::Solution;

fn parse(grid: &str) -> (Vec<usize>, Vec<usize>) {
    let rows: Vec<&str> = grid.lines().collect();
    let num_cols = rows.first().map_or(0, |row| row.len());

    // Convert rows to binary
    let row_ints = rows
        .iter()
        .map(|row| usize::from_str_radix(&row.replace('.', "0").replace('#', "1"), 2).unwrap())
        .collect::<Vec<usize>>();

    // Convert columns to binary
    let col_ints = (0..num_cols)
        .map(|col| {
            usize::from_str_radix(
                &rows
                    .iter()
                    .map(|row| row.chars().nth(col).unwrap())
                    .map(|c| if c == '#' { '1' } else { '0' })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .collect::<Vec<usize>>();

    (row_ints, col_ints)
}

fn reflection_point(slice: &[usize]) -> Option<usize> {
    (1..slice.len()).find(|&i| {
        let left_iter = slice[..i].iter().rev();
        let right_iter = slice[i..].iter();

        left_iter.zip(right_iter).all(|(left, right)| left == right)
    })
}

fn reflection_point_smudged(slice: &[usize]) -> Option<usize> {
    (1..slice.len()).find(|&i| {
        let mut mismatch_count = 0;
        let mut single_bit_difference = false;

        for (left, right) in slice[..i].iter().rev().zip(slice[i..].iter()) {
            if left != right {
                mismatch_count += 1;
                if mismatch_count > 1 || (left ^ right).count_ones() != 1 {
                    return false;
                }
                single_bit_difference = true;
            }
        }

        single_bit_difference
    })
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let grids: Vec<_> = input.split("\n\n").map(parse).collect();

    let part1: usize = grids
        .iter()
        .map(|(rows, cols)| {
            if let Some(row) = reflection_point(&rows) {
                100 * row
            } else if let Some(col) = reflection_point(&cols) {
                col
            } else {
                panic!("No reflection point found");
            }
        })
        .sum();

    let part2 = grids
        .iter()
        .map(|(rows, cols)| {
            if let Some(row) = reflection_point_smudged(&rows) {
                100 * row
            } else if let Some(col) = reflection_point_smudged(&cols) {
                col
            } else {
                println!("{:?} {:?}", rows, cols);
                panic!("No reflection point found");
            }
        })
        .sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day12.txt")) == crate::Solution(0, 0));
    }
}
