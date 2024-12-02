use crate::{Solution, utils::extract_integers};
use itertools::Itertools;
use rhai::{AST, Engine, Scope};

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: AST,
    test: i64,
    if_true: usize,
    if_false: usize,
    inspect_count: usize,
}

fn parse_monkeys(input: &str, engine: &Engine) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();

            lines.next().expect("missing first line");

            let items = extract_integers(lines.next().expect("missing items line"));

            let op = engine
                .compile(
                    lines
                        .next()
                        .expect("missing operation line")
                        .strip_prefix("  Operation: new = ")
                        .expect("invalid operation line"),
                )
                .expect("invalid operation");

            let test = lines
                .next()
                .expect("missing test line")
                .strip_prefix("  Test: divisible by ")
                .expect("invalid test line")
                .parse()
                .expect("invalid test divisor");

            let if_true = lines
                .next()
                .expect("missing true line")
                .strip_prefix("    If true: throw to monkey ")
                .expect("invalid true line")
                .parse()
                .expect("invalid true monkey");

            let if_false = lines
                .next()
                .expect("missing false line")
                .strip_prefix("    If false: throw to monkey ")
                .expect("invalid false line")
                .parse()
                .expect("invalid false monkey");

            Monkey {
                items,
                op,
                test,
                if_true,
                if_false,
                inspect_count: 0,
            }
        })
        .collect()
}

fn run_round(monkeys: &mut Vec<Monkey>, engine: &Engine, lcm: i64, relief: bool) {
    for i in 0..monkeys.len() {
        let items: Vec<_> = monkeys[i].items.drain(..).collect();

        monkeys[i].inspect_count += items.len();

        for item in &items {
            let mut scope = Scope::new();
            scope.push("old", *item);
            let mut result: i64 = engine
                .eval_ast_with_scope(&mut scope, &monkeys[i].op)
                .unwrap();

            if relief {
                result /= 3;
            }

            result %= lcm;

            let to_monkey = if result % monkeys[i].test == 0 {
                monkeys[i].if_true
            } else {
                monkeys[i].if_false
            };

            monkeys[to_monkey].items.push(result);
        }
    }
}

fn run_rounds(
    monkeys: &Vec<Monkey>,
    engine: &Engine,
    lcm: i64,
    rounds: usize,
    relief: bool,
) -> usize {
    let mut monkeys = monkeys.clone();

    for _ in 0..rounds {
        run_round(&mut monkeys, &engine, lcm, relief);
    }

    monkeys
        .into_iter()
        .map(|m| m.inspect_count)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let engine = Engine::new_raw();

    let monkeys = parse_monkeys(input, &engine);

    let lcm = monkeys.iter().map(|m| m.test).fold(1, num::integer::lcm);

    let part1 = run_rounds(&monkeys, &engine, lcm, 20, true);
    let part2 = run_rounds(&monkeys, &engine, lcm, 10_000, false);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day11.txt")) == crate::Solution(10605, 2713310158)
        );
    }
}
