use itertools::Itertools;
use std::{
    fmt::{self, Display},
    io::{self, BufRead},
    ops::Add,
};

#[derive(Clone, Copy, Debug, Default)]
struct Snailfish([Option<u32>; 32]);

fn fmt_slice(slice: &[Option<u32>], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if slice.len() == 1 || slice[slice.len() / 2].is_none() {
        write!(f, "{}", slice[0].unwrap())
    } else {
        write!(f, "[")?;
        fmt_slice(&slice[..slice.len() / 2], f)?;
        write!(f, ",")?;
        fmt_slice(&slice[slice.len() / 2..], f)?;
        write!(f, "]")
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_slice(&self.0, f)
    }
}

impl Snailfish {
    fn parse(s: &str) -> Self {
        let mut v = [None; 32];
        let mut depth = 0;
        let mut i = 0;

        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {}
                _ => {
                    v[i] = c.to_digit(10);
                    i += 1 << (5 - depth);
                }
            }
        }

        Self(v)
    }

    fn explode(&mut self) -> bool {
        for i in (0..32).step_by(2) {
            if let (Some(l), Some(r)) = (self.0[i], self.0[i + 1]) {
                self.0[0..i]
                    .iter_mut()
                    .rev()
                    .flatten()
                    .next()
                    .map(|x| *x += l);

                self.0[i + 2..].iter_mut().flatten().next().map(|x| *x += r);

                self.0[i] = Some(0);
                self.0[i + 1] = None;

                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        for i in 0..32 {
            if let Some(el) = self.0[i] {
                if el >= 10 {
                    self.0[i] = Some(el / 2);

                    for j in (0..i.trailing_zeros().min(5)).rev() {
                        if self.0[i + (1 << j)].is_none() {
                            self.0[i + (1 << j)] = Some((el + 1) / 2);
                            break;
                        }
                    }

                    return true;
                }
            }
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn magnitude_slice(slice: &[Option<u32>]) -> u32 {
        if slice.len() == 1 || slice[slice.len() / 2].is_none() {
            slice[0].unwrap()
        } else {
            3 * Snailfish::magnitude_slice(&slice[..slice.len() / 2])
                + 2 * Snailfish::magnitude_slice(&slice[slice.len() / 2..])
        }
    }

    fn magnitude(self) -> u32 {
        Snailfish::magnitude_slice(&self.0)
    }
}

impl Add<Snailfish> for Snailfish {
    type Output = Snailfish;

    fn add(self, other: Snailfish) -> Snailfish {
        let mut result = Snailfish::default();

        for i in 0..16 {
            result.0[i] = self.0[2 * i];
            result.0[16 + i] = other.0[2 * i];
        }

        result.reduce();

        result
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let data: Vec<Snailfish> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| Snailfish::parse(line.as_str()))
        .collect();

    let total = data
        .iter()
        .cloned()
        .reduce(|acc, i| acc + i)
        .unwrap()
        .magnitude();

    let highest = data
        .into_iter()
        .permutations(2)
        .map(|v| (v[0] + v[1]).magnitude())
        .max()
        .unwrap();

    println!("[Part 1] {:?}", total);
    println!("[Part 2] {:?}", highest);
}
