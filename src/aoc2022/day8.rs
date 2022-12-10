use crate::Solution;
use ndarray::{azip, s, Array2, Ix1, Ix2, SliceInfo, SliceInfoElem, Zip};
use std::cmp::max;

type GridSlice = SliceInfo<[SliceInfoElem; 2], Ix2, Ix1>;

fn sweep(
    grid: &Array2<i8>,
    slices: impl IntoIterator<Item = (GridSlice, GridSlice)>,
) -> Array2<i8> {
    let mut result = Array2::from_elem(grid.dim(), -1i8);

    for (s1, s2) in slices.into_iter() {
        let (prev, next) = result.multi_slice_mut((s1, s2));
        azip!((n in next, &mut p in prev, &g in grid.slice(s1)) *n = max(p, g));
    }

    result
}

fn visibility(grid: &Array2<i8>, slice_fn: fn((usize, usize)) -> GridSlice) -> Array2<usize> {
    Array2::from_shape_fn(grid.dim(), |(i, j)| {
        let slice = grid.slice(slice_fn((i, j)));
        slice
            .iter()
            .position(|&e| e >= grid[(i, j)])
            .map_or(slice.len(), |v| v + 1)
    })
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let lines = input
        .lines()
        .map(|s| {
            s.chars()
                .take_while(|c| c.is_numeric())
                .map(|c| c as i8 - '0' as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let dim = (lines.len(), lines[0].len());

    let grid = Array2::from_shape_vec(dim, lines.iter().flatten().cloned().collect()).unwrap();

    let west = sweep(&grid, (1..dim.1).map(|i| (s![.., i - 1], s![.., i])));
    let east = sweep(
        &grid,
        (0..dim.1 - 1).rev().map(|i| (s![.., i + 1], s![.., i])),
    );
    let north = sweep(&grid, (1..dim.0).map(|i| (s![i - 1, ..], s![i, ..])));
    let south = sweep(
        &grid,
        (0..dim.0 - 1).rev().map(|i| (s![i + 1, ..], s![i, ..])),
    );

    let part1 = Zip::from(&grid)
        .and(&west)
        .and(&east)
        .and(&north)
        .and(&south)
        .fold(0, |acc, g, w, e, n, s| {
            acc + (g > w || g > e || g > n || g > s) as usize
        });

    let west = visibility(&grid, |(i, j)| s![i, ..j;-1]);
    let east = visibility(&grid, |(i, j)| s![i, j + 1..]);
    let north = visibility(&grid, |(i, j)| s![..i;-1, j]);
    let south = visibility(&grid, |(i, j)| s![i + 1.., j]);

    let part2 = (west * east * north * south).into_iter().max().unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day8.txt")) == crate::Solution(21, 8));
    }
}
