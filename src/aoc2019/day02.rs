use super::intcode::Intcode;
use crate::utils::extract_integers;
use crate::Solution;

pub fn solve(input: &str) -> Solution<isize, isize> {
    let ops: Vec<isize> = extract_integers(input);

    let mut ops1 = ops.clone();

    ops1[1] = 12;
    ops1[2] = 2;

    let part1 = Intcode::new(&ops1).0.run().join().unwrap().ops[0];

    for i in 0..100 {
        for j in 0..100 {
            let mut ops2 = ops.clone();

            ops2[1] = i;
            ops2[2] = j;

            let intcode = Intcode::new(&ops2).0.run().join().unwrap();

            if intcode.ops[0] == 19690720 {
                return Solution(part1, 100 * i + j);
            }
        }
    }

    unreachable!();
}
