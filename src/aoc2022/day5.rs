use crate::utils::extract_integers;
use std::io::stdin;

fn move_crates(
    mut stacks: Vec<Vec<char>>,
    moves: &[(usize, usize, usize)],
    preserve_order: bool,
) -> String {
    for &(count, stack_from, stack_to) in moves {
        let split = stacks[stack_from].len() - count;
        let mut moves = stacks[stack_from].split_off(split);

        if !preserve_order {
            moves.reverse();
        }

        stacks[stack_to].extend(moves);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

pub fn solve() {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    for line in stdin().lines().map(Result::unwrap) {
        if line == "" {
            break;
        }

        for (i, j) in (1..line.len()).step_by(4).enumerate() {
            let val = line.as_bytes()[j] as char;

            if val.is_ascii_alphabetic() {
                stacks[i].insert(0, val);
            }
        }
    }

    let moves = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let action = extract_integers(&l);

            (
                action[0] as usize,
                (action[1] - 1) as usize,
                (action[2] - 1) as usize,
            )
        })
        .collect::<Vec<(usize, usize, usize)>>();

    println!("part1: {}", move_crates(stacks.clone(), &moves, false));
    println!("part1: {}", move_crates(stacks, &moves, true));
}
