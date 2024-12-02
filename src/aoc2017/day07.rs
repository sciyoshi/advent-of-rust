use crate::Solution;
use itertools::Itertools;
use petgraph::Graph;
use petgraph::algo::toposort;
use regex::Regex;
use std::collections::HashMap;

struct Node {
    name: String,
    weight: u32,
    total: u32,
}

pub fn solve(input: &str) -> Solution<String, u32> {
    let re = Regex::new("[[:word:]]+").unwrap();

    // Read into an array of the form ["node", "(weight)", "child", ...]
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|l| re.find_iter(l).map(|m| m.as_str().to_string()).collect())
        .collect();

    // Build a graph
    let mut indices = HashMap::new();
    let mut graph = Graph::<Node, ()>::new();

    // First, insert all the nodes
    for line in &lines {
        let weight = line[1].parse::<u32>().unwrap();
        let node = Node {
            name: line[0].to_string(),
            weight,
            total: weight,
        };
        let idx = graph.add_node(node);
        indices.insert(line[0].to_string(), idx);
    }

    // Now add all the edges
    for line in &lines {
        for child in &line[2..] {
            graph.add_edge(indices[&line[0]], indices[child], ());
        }
    }

    // Topological sort to find ordering of the nodes from the root
    let sorted = toposort(&graph, None).unwrap();

    let part1 = graph[sorted[0]].name.to_string();

    // Now, find the unbalanced node, starting at the leaves...
    for &node in sorted.iter().rev() {
        // If this node's children aren't all equal
        if !graph.neighbors(node).map(|n| graph[n].total).all_equal() {
            // Find the min and max value of the children
            let (min, max) = graph
                .neighbors(node)
                .map(|n| graph[n].total)
                .minmax()
                .into_option()
                .unwrap();

            // Split the children based on their total (left for min, right for max)
            let (left, right): (Vec<_>, Vec<_>) =
                graph.neighbors(node).partition(|&n| graph[n].total == min);

            // The unbalanced node is the side that has one element
            let unbalanced = if left.len() == 1 {
                &graph[left[0]]
            } else {
                &graph[right[0]]
            };

            // Find that node's new weight in order to balance the weights
            return Solution(part1, unbalanced.weight + min - max);
        }

        // Otherwise, update this node's total weight
        graph[node].total += graph.neighbors(node).map(|n| graph[n].total).sum::<u32>();
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day07.txt"))
                == crate::Solution("tknk".to_string(), 60)
        );
    }
}
