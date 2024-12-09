use crate::Solution;

fn compact(disk: &mut [Option<usize>]) {
    let mut start = 0;
    let mut end = disk.len() - 1;

    while disk[start].is_some() {
        start += 1;
    }

    while disk[end].is_none() {
        end -= 1;
    }

    while start < end {
        disk.swap(start, end);

        start += 1;
        end -= 1;

        while disk[start].is_some() {
            start += 1;
        }

        while disk[end].is_none() {
            end -= 1;
        }
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut disk = vec![];
    let mut pos = 0usize;
    let mut file = 0;
    let mut adding = true;
    let mut files = vec![];
    let mut spaces = vec![];

    for c in input.chars().filter(|&c| char::is_digit(c, 10)) {
        let num = c.to_digit(10).unwrap() as usize;

        for _ in 0..num {
            disk.push(if adding { Some(file) } else { None });
        }

        if adding {
            files.push((num, pos));
            file += 1;
        } else {
            spaces.push((num, pos));
        }

        adding = !adding;
        pos += num;
    }

    let mut disk2 = disk.clone();
    compact(&mut disk2);

    let part1 = disk2
        .iter()
        .enumerate()
        .flat_map(|(i, e)| e.map(|d| i * d))
        .sum();

    for (i, &(fsize, fpos)) in files.iter().enumerate().rev() {
        // find first space that fits the file
        let space = spaces[..i]
            .iter()
            .enumerate()
            .find(|&(_, s)| s.0 >= fsize)
            .map(|(i, _)| i);

        if let Some(space) = space {
            let (ssize, spos) = spaces[space];

            disk[spos..spos + fsize].fill(Some(i));
            disk[fpos..fpos + fsize].fill(None);

            spaces[space] = (ssize - fsize, spos + fsize);
        }
    }

    let part2 = disk
        .iter()
        .enumerate()
        .flat_map(|(i, e)| e.map(|d| i * d))
        .sum();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day08.txt")) == crate::Solution(1928, 2858));
    }
}
