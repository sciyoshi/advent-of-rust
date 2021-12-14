use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn apply(
    rules: &HashMap<(char, char), char>,
    pairs: HashMap<(char, char), u64>,
) -> HashMap<(char, char), u64> {
    let mut new_pairs = HashMap::new();

    for ((c1, c2), v) in pairs {
        if rules.contains_key(&(c1, c2)) {
            *new_pairs.entry((c1, rules[&(c1, c2)])).or_insert(0) += v;
            *new_pairs.entry((rules[&(c1, c2)], c2)).or_insert(0) += v;
        } else {
            *new_pairs.entry((c1, c2)).or_insert(0) += v;
        }
    }

    new_pairs
}

fn score(template: &str, pairs: &HashMap<(char, char), u64>) -> u64 {
    let mut counts = HashMap::new();

    counts.insert(template.chars().nth(0).unwrap(), 1);

    for (&(_c1, c2), v) in pairs {
        *counts.entry(c2).or_insert(0) += v;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let template = data[0].as_str();
    let rules: HashMap<(char, char), char> = data[2..]
        .iter()
        .map(|l| l.as_bytes())
        .map(|l| ((l[0] as char, l[1] as char), l[6] as char))
        .collect();

    let mut pairs = HashMap::new();

    for (c1, c2) in template.chars().tuple_windows() {
        *pairs.entry((c1, c2)).or_insert(0) += 1;
    }

    for _ in 0..10 {
        pairs = apply(&rules, pairs);
    }

    println!("[Part 1] {:?}", score(template, &pairs));

    for _ in 10..40 {
        pairs = apply(&rules, pairs);
    }

    println!("[Part 2] {:?}", score(template, &pairs));
}
