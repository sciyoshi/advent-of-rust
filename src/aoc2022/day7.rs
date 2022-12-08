use crate::Solution;
use std::iter::{self, Iterator};

struct Dir {
    size: usize,
    children: Vec<Dir>,
}

fn parse(commands: &mut impl Iterator<Item = &str>) -> (Dir, usize) {
    if commands.next().unwrap() != "$ ls" {
        panic!("need ls command first");
    }

    let mut children: Vec<Dir> = vec![];
    let mut size = 0;

    let mut part1 = 0;

    while let Some(line) = commands.next() {
        if line.starts_with("dir ") {
            // handled by cd
        } else if line == "$ cd .." {
            break;
        } else if line.starts_with("$ cd ") {
            let (subdir, subpart1) = parse(commands);
            size += subdir.size;
            part1 += subpart1;
            children.push(subdir);
        } else {
            size += line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
    }

    if size <= 100000 {
        part1 += size;
    }

    (Dir { size, children }, part1)
}

fn directory_sizes(dir: &Dir) -> Box<dyn Iterator<Item = usize> + '_> {
    box iter::once(dir.size).chain(
        dir.children
            .iter()
            .flat_map(|subdir| directory_sizes(subdir)),
    )
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut lines = input.lines();

    if lines.next().unwrap() != "$ cd /" {
        panic!("need cd / command first");
    }

    let (result, part1) = parse(&mut lines);

    let part2 = directory_sizes(&result)
        .filter(|&size| size > result.size - 40000000)
        .min()
        .unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day7.txt")) == crate::Solution(95437, 24933642)
        );
    }
}
