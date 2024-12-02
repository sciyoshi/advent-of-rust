use crate::{Solution, utils::extract_integers};
use itertools::Itertools;

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

pub fn solve(input: &str) -> Solution<String, String> {
    let mut lines = input.lines().peekable();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; lines.peek().unwrap().len() / 4 + 1];

    for line in lines.take_while_ref(|line| !line.is_empty()) {
        for (i, j) in (1..line.len()).step_by(4).enumerate() {
            let val = line.as_bytes()[j] as char;

            if val.is_ascii_alphabetic() {
                stacks[i].insert(0, val);
            }
        }
    }

    let moves = lines
        .skip(1)
        .map(|l| {
            let action = extract_integers::<usize>(&l);

            (action[0], action[1] - 1, action[2] - 1)
        })
        .collect::<Vec<(usize, usize, usize)>>();

    Solution(
        move_crates(stacks.clone(), &moves, false),
        move_crates(stacks, &moves, true),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day5.txt"))
                == crate::Solution("CMZ".to_string(), "MCD".to_string())
        );
    }
}
