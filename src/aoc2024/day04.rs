use crate::Solution;

use crate::util::euclid::{Box2, Pt2, Pt2Ext, Vec2, Vec2Ext, pt2};

pub fn check(grid: &[Vec<char>], pt: Pt2<isize>, dir: Vec2<isize>) -> bool {
    (pt + dir * 3).within(Box2::new(
        pt2(0, 0),
        pt2(grid[0].len() as isize, grid.len() as isize),
    )) && grid[pt.y as usize][pt.x as usize] == 'X'
        && grid[(pt.y + dir.y) as usize][(pt.x + dir.x) as usize] == 'M'
        && grid[(pt.y + dir.y * 2) as usize][(pt.x + dir.x * 2) as usize] == 'A'
        && grid[(pt.y + dir.y * 3) as usize][(pt.x + dir.x * 3) as usize] == 'S'
}

pub fn check_x(grid: &[Vec<char>], pt: Pt2<isize>) -> bool {
    let bounds = Box2::new(pt2(0, 0), pt2(grid[0].len() as isize, grid.len() as isize));

    (pt + Vec2::nw()).within(bounds)
        && (pt + Vec2::se()).within(bounds)
        && grid[pt.y as usize][pt.x as usize] == 'A'
        && ((grid[(pt.y + 1) as usize][(pt.x + 1) as usize] == 'S'
            && grid[(pt.y - 1) as usize][(pt.x - 1) as usize] == 'M')
            || (grid[(pt.y + 1) as usize][(pt.x + 1) as usize] == 'M'
                && grid[(pt.y - 1) as usize][(pt.x - 1) as usize] == 'S'))
        && ((grid[(pt.y + 1) as usize][(pt.x - 1) as usize] == 'S'
            && grid[(pt.y - 1) as usize][(pt.x + 1) as usize] == 'M')
            || (grid[(pt.y + 1) as usize][(pt.x - 1) as usize] == 'M'
                && grid[(pt.y - 1) as usize][(pt.x + 1) as usize] == 'S'))
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let pt: Pt2<isize> = pt2(i as isize, j as isize);

            for dir in &[
                Vec2Ext::n(),
                Vec2Ext::e(),
                Vec2Ext::s(),
                Vec2Ext::w(),
                Vec2Ext::ne(),
                Vec2Ext::se(),
                Vec2Ext::nw(),
                Vec2Ext::sw(),
            ] {
                if check(&grid, pt, *dir) {
                    part1 += 1;
                }
            }

            if check_x(&grid, pt) {
                part2 += 1;
            }
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day04.txt")) == crate::Solution(18, 9));
    }
}
