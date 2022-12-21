use crate::Solution;
use std::collections::BTreeMap;
// use regex::Regex;
// use rhai::Engine;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Monkey<'a> {
    Human(isize),
    Num(isize),
    BinOp(Op, &'a str, &'a str),
}

#[derive(Debug)]
struct Node<'a> {
    monkey: Monkey<'a>,
    has_human: bool,
}

fn eval(monkeys: &BTreeMap<&str, Node>, node: &str) -> isize {
    match &monkeys[node].monkey {
        &Monkey::Human(v) | &Monkey::Num(v) => v,
        &Monkey::BinOp(Op::Add, l, r) => eval(monkeys, l) + eval(monkeys, r),
        &Monkey::BinOp(Op::Sub, l, r) => eval(monkeys, l) - eval(monkeys, r),
        &Monkey::BinOp(Op::Mul, l, r) => eval(monkeys, l) * eval(monkeys, r),
        &Monkey::BinOp(Op::Div, l, r) => eval(monkeys, l) / eval(monkeys, r),
    }
}

fn find_human(monkeys: &mut BTreeMap<&str, Node>, node: &str) -> bool {
    let has_human = match &monkeys[node].monkey {
        &Monkey::Human(_) => true,
        &Monkey::Num(_) => false,
        &Monkey::BinOp(_, l, r) => find_human(monkeys, l) || find_human(monkeys, r),
    };

    monkeys.get_mut(node).unwrap().has_human = has_human;

    has_human
}

fn rotate(monkeys: &mut BTreeMap<&str, Node>) -> isize {
    if let &Monkey::BinOp(_, mut left, mut right) = &monkeys["root"].monkey {
        if monkeys[right].has_human {
            (right, left) = (left, right);
        }

        loop {
            match &monkeys[left].monkey {
                &Monkey::Human(_) => {
                    break;
                }
                &Monkey::Num(_) => unreachable!(),
                &Monkey::BinOp(op, x, y) => {
                    let x_monkey = monkeys[x].has_human;
                    let left_monkey = monkeys.get_mut(left).unwrap();
                    let y_monkey = if x_monkey { y } else { x };
                    left_monkey.monkey = match op {
                        Op::Add => Monkey::BinOp(Op::Sub, right, y_monkey),
                        Op::Sub => {
                            Monkey::BinOp(if x_monkey { Op::Add } else { Op::Sub }, y_monkey, right)
                        }
                        Op::Mul => Monkey::BinOp(Op::Div, right, y_monkey),
                        Op::Div => {
                            Monkey::BinOp(if x_monkey { Op::Mul } else { Op::Div }, y_monkey, right)
                        }
                    };

                    right = left;
                    left = if x_monkey { x } else { y };
                }
            }
        }

        eval(monkeys, right)
    } else {
        unreachable!();
    }
}

pub fn solve(input: &str) -> Solution<isize, isize> {
    let mut monkeys = BTreeMap::new();

    input.lines().for_each(|l| {
        let name = &l[..4];

        if let Ok(val) = l[6..].parse::<isize>() {
            monkeys.insert(
                name,
                Node {
                    monkey: if name == "humn" {
                        Monkey::Human(val)
                    } else {
                        Monkey::Num(val)
                    },
                    has_human: name == "humn",
                },
            );
        } else {
            let left = &l[6..10];
            let right = &l[13..17];

            monkeys.insert(
                name,
                Node {
                    monkey: match &l[11..12] {
                        "+" => Monkey::BinOp(Op::Add, left, right),
                        "-" => Monkey::BinOp(Op::Sub, left, right),
                        "*" => Monkey::BinOp(Op::Mul, left, right),
                        "/" => Monkey::BinOp(Op::Div, left, right),
                        _ => panic!(),
                    },
                    has_human: false,
                },
            );
        }
    });

    find_human(&mut monkeys, "root");

    let part1 = eval(&monkeys, "root");
    let part2 = rotate(&mut monkeys);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day21.txt")) == crate::Solution(152, 301));
    }
}
