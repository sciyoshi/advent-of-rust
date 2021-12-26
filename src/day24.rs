use std::{
    io::{self, BufRead},
    mem::take,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Expr {
    Input(usize),
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Eql(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn value_range(&self) -> (i64, i64) {
        match self {
            Expr::Input(_) => (0, 9),
            Expr::Const(a) => (*a, *a),
            Expr::Add(left, right) => {
                let left = left.value_range();
                let right = right.value_range();
                (left.0 + right.0, left.1 + right.1)
            }
            Expr::Mul(left, right) => {
                let left = left.value_range();
                let right = right.value_range();
                (left.0 * right.0, left.1 * right.1)
            }
            Expr::Div(left, right) => {
                let left = left.value_range();
                let right = right.value_range();
                (left.0 / right.1, left.1 / right.0)
            }
            Expr::Mod(_, right) => {
                let right = right.value_range();
                (0, right.1)
            }
            Expr::Eql(_, _) => (0, 1),
        }
    }

    fn simplify(&mut self, constraints: &mut Vec<(usize, i64, usize)>) {
        loop {
            let mut changed = false;
            match self {
                Expr::Mul(left, right) => {
                    if left.as_ref() == &Expr::Const(0) || right.as_ref() == &Expr::Const(0) {
                        *self = Expr::Const(0);
                        changed = true;
                    } else if left.as_ref() == &Expr::Const(1) {
                        *self = take(right);
                        changed = true;
                    } else if right.as_ref() == &Expr::Const(1) {
                        *self = take(left);
                        changed = true;
                    }
                }
                Expr::Add(box Expr::Const(left), box Expr::Const(right)) => {
                    *self = Expr::Const(*left + *right);
                    changed = true;
                }
                Expr::Add(box Expr::Add(box left, box Expr::Const(v1)), box Expr::Const(v2)) => {
                    *self = Expr::Add(Box::new(take(left)), Box::new(Expr::Const(*v1 + *v2)));
                    changed = true;
                }
                Expr::Add(left, right) => {
                    if left.as_ref() == &Expr::Const(0) {
                        *self = take(right);
                        changed = true;
                    } else if right.as_ref() == &Expr::Const(0) {
                        *self = take(left);
                        changed = true;
                    }
                }
                Expr::Div(left, right) => match (left.as_ref(), right.as_ref()) {
                    (Expr::Const(a), Expr::Const(b)) => {
                        *self = Expr::Const(a / b);
                        changed = true;
                    }
                    (_, Expr::Const(1)) => {
                        *self = take(left);
                        changed = true;
                    }
                    (
                        Expr::Add(box Expr::Mul(box v, box Expr::Const(b2)), box y),
                        Expr::Const(b1),
                    ) => {
                        if b1 == b2 && y.value_range().1 < *b1 {
                            *self = v.clone();
                            changed = true;
                        }
                    }
                    (_, Expr::Const(b)) => {
                        if left.value_range().1 < *b && left.value_range().0 >= 0 {
                            *self = Expr::Const(0);
                            changed = true;
                        }
                    }
                    _ => {}
                },
                Expr::Mod(
                    box Expr::Add(box Expr::Mul(_, box Expr::Const(b2)), box v),
                    box Expr::Const(b1),
                ) => {
                    if b1 == b2 {
                        *self = take(v);
                        changed = true;
                    }
                }
                Expr::Mod(left, right) => match (left.as_ref(), right.as_ref()) {
                    (Expr::Const(a), Expr::Const(b)) => {
                        *self = Expr::Const(a % b);
                        changed = true;
                    }
                    (_, Expr::Const(b)) => {
                        if left.value_range().1 < *b {
                            *self = take(left);
                            changed = true;
                        }
                    }
                    _ => {}
                },
                Expr::Eql(left, right) => match (left.as_ref(), right.as_ref()) {
                    (Expr::Add(box Expr::Input(l), box Expr::Const(v)), Expr::Input(i)) => {
                        if *v > 9 {
                            *self = Expr::Const(0);
                        } else {
                            constraints.push((*l, *v, *i));
                            *self = Expr::Const(1);
                        }
                        changed = true;
                    }
                    (_, Expr::Input(_)) => {
                        if left.value_range().0 > 9 || left.value_range().1 < 0 {
                            *self = Expr::Const(0);
                            changed = true;
                        }
                    }
                    (Expr::Input(_), _) => {
                        if right.value_range().0 > 9 || right.value_range().1 < 0 {
                            *self = Expr::Const(0);
                            changed = true;
                        }
                    }
                    (Expr::Const(a), Expr::Const(b)) => {
                        *self = Expr::Const((a == b) as i64);
                        changed = true;
                    }
                    _ => {}
                },
                _ => {}
            }
            if !changed {
                break;
            }
        }
    }
}

impl Default for Expr {
    fn default() -> Self {
        Expr::Const(0)
    }
}

fn register_index(s: &str) -> usize {
    match s {
        "x" => 0,
        "y" => 1,
        "z" => 2,
        "w" => 3,
        _ => panic!("invalid register"),
    }
}

pub fn solve() {
    let data: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let mut input = 0;
    let mut registers = [
        Expr::Const(0),
        Expr::Const(0),
        Expr::Const(0),
        Expr::Const(0),
    ];

    let mut constraints = vec![];

    for line in &data {
        let inst: Vec<&str> = line.split_whitespace().collect();

        match inst[0] {
            "inp" => {
                registers[register_index(inst[1])] = Expr::Input(input);
                input += 1;
            }
            _ => {
                let reg = register_index(inst[1]);
                let arg1 = take(&mut registers[reg]);
                let arg2 = if let Ok(value) = inst[2].parse::<i64>() {
                    Box::new(Expr::Const(value))
                } else {
                    Box::new(registers[register_index(inst[2])].clone())
                };

                registers[reg] = match inst[0] {
                    "add" => Expr::Add(Box::new(arg1), arg2),
                    "mul" => Expr::Mul(Box::new(arg1), arg2),
                    "div" => Expr::Div(Box::new(arg1), arg2),
                    "mod" => Expr::Mod(Box::new(arg1), arg2),
                    "eql" => Expr::Eql(Box::new(arg1), arg2),
                    _ => panic!("invalid instruction"),
                };

                registers[reg].simplify(&mut constraints);
            }
        }
    }

    let mut max = vec![0; 14];
    let mut min = vec![0; 14];

    for (i, diff, j) in constraints {
        if diff < 0 {
            max[i] = 9;
            max[j] = 9 + diff;
            min[i] = 1 - diff;
            min[j] = 1;
        } else {
            max[i] = 9 - diff;
            max[j] = 9;
            min[i] = 1;
            min[j] = 1 + diff;
        }
    }

    println!(
        "[Part 1] {}",
        max.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
    println!(
        "[Part 2] {}",
        min.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
