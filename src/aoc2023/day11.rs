// Generated with ChatGPT 4.
// https://chat.openai.com/share/d6b97ffd-3ffc-4b49-908b-01f44a3c0066

use crate::Solution;

fn parse(input: &str) -> Vec<(isize, isize)> {
    let mut points = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                points.push((i as isize, j as isize));
            }
        }
    }

    points
}

fn expand_points(mut points: Vec<(isize, isize)>, offset_increment: isize) -> Vec<(isize, isize)> {
    // Sort points by row
    points.sort_by_key(|&(row, _)| row);

    // Adjust row coordinates
    let mut offset = 0;
    let mut last_row = -1;
    for point in points.iter_mut() {
        if point.0 > last_row {
            offset += (point.0 - last_row - 1) * offset_increment;
            last_row = point.0;
        }
        point.0 += offset;
    }

    // Sort points by column
    points.sort_by_key(|&(_, col)| col);

    // Adjust column coordinates
    offset = 0;
    let mut last_col = -1;
    for point in points.iter_mut() {
        if point.1 > last_col {
            offset += (point.1 - last_col - 1) * offset_increment;
            last_col = point.1;
        }
        point.1 += offset;
    }

    points
}

fn sum_of_manhattan_distances(points: &[(isize, isize)]) -> isize {
    let mut sum = 0;

    for (i, &point1) in points.iter().enumerate() {
        for &point2 in &points[i + 1..] {
            sum += (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs();
        }
    }

    sum
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let points = parse(input);

    let expanded_points_1 = expand_points(points.clone(), 1);
    let distance_sum_1 = sum_of_manhattan_distances(&expanded_points_1);

    let expanded_points_999999 = expand_points(points, 999_999);
    let distance_sum_999999 = sum_of_manhattan_distances(&expanded_points_999999);

    Solution(distance_sum_1, distance_sum_999999)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day11.txt")) == crate::Solution(374, 82000210));
    }
}
