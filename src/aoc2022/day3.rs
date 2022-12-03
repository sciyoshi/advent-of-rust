use std::collections::BTreeSet;
use std::io::stdin;

fn priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        item as u32 - 'A' as u32 + 27
    } else {
        item as u32 - 'a' as u32 + 1
    }
}

pub fn solve() -> (u32, u32) {
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();

    let part1 = lines
        .iter()
        .map(|l| {
            let set1 = l[..l.len() / 2].chars().collect::<BTreeSet<char>>();
            let set2 = l[l.len() / 2..].chars().collect::<BTreeSet<char>>();

            priority(*set1.iter().filter(|c| set2.contains(c)).next().unwrap())
        })
        .sum();

    let part2 = lines
        .array_chunks::<3>()
        .map(|l| {
            let set1 = l[0].chars().collect::<BTreeSet<char>>();
            let set2 = l[1].chars().collect::<BTreeSet<char>>();
            let set3 = l[2].chars().collect::<BTreeSet<char>>();

            priority(
                *set1
                    .iter()
                    .filter(|c| set2.contains(c) && set3.contains(c))
                    .next()
                    .unwrap(),
            )
        })
        .sum();

    (part1, part2)
}
