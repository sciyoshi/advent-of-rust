use crate::Solution;
use crate::utils::Pt;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution<String, usize> {
    let mut map: HashMap<Pt<i64>, char> = HashMap::new();
    let mut pos = Pt(0, 0);
    let mut dir = Pt::e();
    let mut steps = 0;
    let mut path = vec![];

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map.insert(Pt(i as i64, j as i64), c);

            if i == 0 && c == '|' {
                pos = Pt(i as i64, j as i64);
            }
        }
    }

    while let Some(c) = map.get(&pos) {
        match c {
            ' ' => break,
            'A'..='Z' => path.push(c),
            '+' => {
                if *map.get(&(pos + dir.rot90r())).unwrap_or(&' ') != ' ' {
                    dir = dir.rot90r();
                } else {
                    dir = dir.rot90l();
                }
            }
            _ => {}
        }
        pos += dir;
        steps += 1;
    }

    Solution(path.into_iter().collect::<String>(), steps)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day19.txt"))
                == crate::Solution("ABCDEF".to_string(), 38)
        );
    }
}
