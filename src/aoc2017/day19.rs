use crate::util::Pt;
use crate::Solution;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution<i64, i64> {
    let stdin = io::stdin();

    let mut map: HashMap<Pt, char> = HashMap::new();
    let mut pos = Pt(0, 0);
    let mut dir = Pt::e();
    let mut steps = 0;
    let mut path = vec![];

    for (i, line) in stdin.lock().lines().enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            map.insert(Pt(i as isize, j as isize), c);

            if i == 0 && c == '|' {
                pos = Pt(i as isize, j as isize);
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
        pos = pos + dir;
        steps += 1;
    }

    println!("[Part 1] Path: {}", path.into_iter().collect::<String>());
    println!("[Part 2] Steps: {}", steps);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
