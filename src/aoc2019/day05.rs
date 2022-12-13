use crate::utils::extract_integers;
use crate::Solution;

use super::intcode::Intcode;

pub fn solve(input: &str) -> Solution<isize, isize> {
    let ops: Vec<_> = extract_integers::<isize>(input).into_iter().collect();

    let (code, input, output) = Intcode::new(&ops);
    code.run();
    input.send(1).unwrap();
    let mut part1 = 0;
    while let Ok(out) = output.recv() {
        part1 = out;
    }

    let (code, input, output) = Intcode::new(&ops);
    code.run();
    input.send(5).unwrap();
    let mut part2 = 0;
    while let Ok(out) = output.recv() {
        part2 = out;
    }

    Solution(part1, part2)
}
