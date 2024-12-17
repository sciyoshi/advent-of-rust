use itertools::Itertools;

use crate::{Solution, utils::extract_integers};

#[derive(Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Inst {
    op: Op,
    arg: usize,
}

impl Inst {
    fn combo(&self, registers: &[usize]) -> usize {
        match self.arg {
            v @ 0..=3 => v,
            v @ 4..=6 => registers[v - 4],
            _ => panic!("invalid operand"),
        }
    }

    fn execute(&self, ip: usize, registers: &mut [usize], output: &mut Vec<usize>) -> usize {
        match self.op {
            Op::Adv => {
                registers[0] /= 1 << self.combo(registers);
            }
            Op::Bxl => {
                registers[1] ^= self.arg;
            }
            Op::Bst => {
                registers[1] = self.combo(registers) % 8;
            }
            Op::Jnz => {
                if registers[0] != 0 {
                    return self.arg;
                }
            }
            Op::Bxc => {
                registers[1] ^= registers[2];
            }
            Op::Out => {
                output.push(self.combo(registers) % 8);
            }
            Op::Bdv => {
                registers[1] = registers[0] / (1 << self.combo(registers));
            }
            Op::Cdv => {
                registers[2] = registers[0] / (1 << self.combo(registers));
            }
        }

        ip + 2
    }
}

// implements this in rust
// def inverse(a, p):
//     if not p:
//         return a
//     for i in reversed(range(8)):
//         b = i ^ 6
//         b = b ^ ((8 * a + i) >> b) ^ 4
//         if b % 8 == p[0]:
//             try:
//                 return solve(8 * a + i, p[1:])
//             except ValueError:
//                 continue
//     else:
//         raise ValueError
fn inverse(a: usize, program: &[usize]) -> Option<usize> {
    if program.is_empty() {
        return Some(a);
    }

    for i in 0..8 {
        let b = i ^ 6;
        let b = b ^ ((8 * a + i) >> b) ^ 4;
        if b % 8 == program[program.len() - 1] {
            match inverse(8 * a + i, &program[..program.len() - 1]) {
                Some(v) => return Some(v),
                None => continue,
            }
        }
    }

    None
}

pub fn solve(input: &str) -> Solution<String, usize> {
    let (mut registers, program) = input
        .split("\n\n")
        .map(extract_integers::<usize>)
        .collect_tuple()
        .unwrap();

    let ops = program
        .iter()
        .array_chunks()
        .map(|[inst, operand]| Inst {
            op: match inst {
                0 => Op::Adv,
                1 => Op::Bxl,
                2 => Op::Bst,
                3 => Op::Jnz,
                4 => Op::Bxc,
                5 => Op::Out,
                6 => Op::Bdv,
                7 => Op::Cdv,
                _ => panic!(),
            },
            arg: *operand,
        })
        .collect_vec();

    let mut ip = 0;
    let mut output = vec![];

    while ip / 2 < ops.len() {
        ip = ops[ip / 2].execute(ip, &mut registers, &mut output);
    }

    let part1 = output.iter().map(usize::to_string).join(",");
    let part2 = inverse(0, &program).unwrap_or(0);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day17.txt"))
                == crate::Solution("4,6,3,5,6,3,5,2,1,0".to_string(), 0)
        );
    }
}
