use std::sync::mpsc;
use std::thread;

use super::intcode::Intcode;
use crate::Solution;
use crate::utils::extract_integers;
use itertools::Itertools;

fn simulate(ops: &Vec<isize>, phases: impl IntoIterator<Item = isize>) -> isize {
    let (mut pipe_tx, mut pipe_rx) = mpsc::channel();
    let first_tx = pipe_tx.clone();

    for phase in phases.into_iter() {
        let (next_tx, next_rx) = mpsc::channel();

        pipe_tx.send(phase).unwrap();

        Intcode::new_with_io(ops, pipe_rx, next_tx.clone()).run();

        (pipe_tx, pipe_rx) = (next_tx, next_rx);
    }

    first_tx.send(0).unwrap();

    thread::spawn(move || {
        loop {
            let out = pipe_rx.recv().unwrap();
            if let Err(_) = first_tx.send(out) {
                return out;
            }
        }
    })
    .join()
    .unwrap()
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let ops: Vec<isize> = extract_integers(input);

    let part1 = (0..5)
        .permutations(5)
        .map(|phases| simulate(&ops, phases))
        .max()
        .unwrap();

    let part2 = (5..10)
        .permutations(5)
        .map(|phases| simulate(&ops, phases))
        .max()
        .unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
                == crate::Solution(43210, 98765)
        );
    }
}
