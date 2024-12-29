use arrayvec::ArrayString;
use itertools::Itertools;
use petgraph::prelude::DiGraphMap;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

pub fn solve(input: &str) -> Solution<usize, String> {
    let mut graph = DiGraphMap::<ArrayString<3>, ()>::new();
    let mut gate_map = HashMap::<ArrayString<3>, Gate>::new();

    let (init, gates) = input.split("\n\n").collect_tuple().unwrap();

    gates.lines().for_each(|line| {
        let parts = line.split_ascii_whitespace().collect_vec();
        let a = ArrayString::from(parts[0]).unwrap();
        let b = ArrayString::from(parts[2]).unwrap();
        let c = ArrayString::from(parts[4]).unwrap();

        let gate = match parts[1] {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => panic!("Unknown gate"),
        };

        gate_map.insert(c, gate);

        graph.add_edge(a, c, ());
        graph.add_edge(b, c, ());
    });

    let mut values = HashMap::<ArrayString<3>, bool>::new();

    for line in init.lines() {
        let parts = line.split(": ").collect_vec();
        let wire = ArrayString::from(parts[0]).unwrap();
        let value: bool = parts[1].parse::<usize>().unwrap() != 0;
        values.insert(wire, value);
    }

    for node in petgraph::algo::toposort(&graph, None).unwrap() {
        if !values.contains_key(&node) {
            let inputs = graph.neighbors_directed(node, petgraph::Direction::Incoming);

            values.insert(node, match gate_map[&node] {
                Gate::And => {
                    let mut result = true;
                    for input in inputs {
                        result &= values[&input];
                    }
                    result
                }
                Gate::Or => {
                    let mut result = false;
                    for input in inputs {
                        result |= values[&input];
                    }
                    result
                }
                Gate::Xor => {
                    let mut result = false;
                    for input in inputs {
                        result ^= values[&input];
                    }
                    result
                }
            });
        }
    }

    let part1 = values
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .map(|(_, v)| v)
        .fold(0, |acc, &v| (acc << 1) | v as usize);

    let mut bad = HashSet::new();

    for (&key, &gate) in &gate_map {
        let inputs = graph
            .neighbors_directed(key, petgraph::Direction::Incoming)
            .sorted()
            .collect_vec();

        let outputs = graph
            .neighbors_directed(key, petgraph::Direction::Outgoing)
            .sorted()
            .collect_vec();

        if key.starts_with("z") && gate != Gate::Xor && key.as_str() != "z45" {
            bad.insert(key);
        }

        if gate == Gate::Xor
            && !key.starts_with("z")
            && !(inputs[0].starts_with("x") && inputs[1].starts_with("y"))
        {
            bad.insert(key);
        }

        if gate == Gate::And
            && inputs[0].as_str() != "x00"
            && outputs.iter().any(|&o| gate_map[o.as_str()] != Gate::Or)
        {
            bad.insert(key);
        }

        if gate == Gate::Xor && outputs.iter().any(|&o| gate_map[o.as_str()] == Gate::Or) {
            bad.insert(key);
        }
    }

    Solution(part1, bad.iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day24.txt")).0 == 2024);
    }
}
