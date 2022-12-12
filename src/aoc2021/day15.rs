use crate::util::Pt;
use crate::Solution;
use petgraph::{algo::astar, graphmap::DiGraphMap};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let grid: Vec<Vec<u32>> = data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut graph = DiGraphMap::<Pt<isize>, u32>::new();

    let (w, h) = (grid.len() as isize, grid[0].len() as isize);

    for i in 0..w {
        for j in 0..h {
            let pt = Pt(i, j);
            for nb in pt.nb4() {
                if nb.within(0, 0, w - 1, h - 1) {
                    graph.add_edge(pt, nb, grid[nb.0 as usize][nb.1 as usize]);
                }
            }
        }
    }

    let (part1, _) = astar(
        &graph,
        Pt(0, 0),
        |pt| pt == Pt(w - 1, h - 1),
        |e| *e.2,
        |_| 0,
    )
    .unwrap();

    println!("[Part 1] {:?}", part1);

    let mut graph = DiGraphMap::<Pt<isize>, u32>::new();

    for i in 0..(5 * w) {
        for j in 0..(5 * h) {
            let pt = Pt(i as isize, j as isize);
            for nb in pt.nb4() {
                if nb.within(0, 0, 5 * w - 1, 5 * h - 1) {
                    let wt = grid[(nb.0 % w) as usize][(nb.1 % h) as usize];
                    graph.add_edge(pt, nb, (wt + (nb.0 / w + nb.1 / h) as u32 - 1) % 9 + 1);
                }
            }
        }
    }

    let (part2, _) = astar(
        &graph,
        Pt(0, 0),
        |pt| pt == Pt(5 * w - 1, 5 * h - 1),
        |e| *e.2,
        |_| 0,
    )
    .unwrap();

    println!("[Part 2] {:?}", part2);
}
