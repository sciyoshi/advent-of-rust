use crate::Solution;
use itertools::Itertools;
use petgraph::graph::Graph;
use petgraph::Undirected;
use rustworkx_core::centrality::edge_betweenness_centrality;
use std::collections::HashMap;

fn parse(input: &str) -> Graph<&str, (), Undirected> {
    let mut graph = Graph::<&str, (), Undirected>::new_undirected();
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }

        let node = parts[0].trim();
        let connections = parts[1].split_whitespace();

        let node_index = *nodes.entry(node).or_insert_with(|| graph.add_node(node));

        for connection in connections {
            let connection_index = *nodes
                .entry(connection)
                .or_insert_with(|| graph.add_node(connection));
            graph.add_edge(node_index, connection_index, ());
        }
    }

    graph
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut graph = parse(input);

    let centrality = edge_betweenness_centrality(&graph, true, 100);

    // find largest 3 elements of centrality
    let top = centrality
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
        .take(3)
        .map(|(i, _)| i)
        .collect_vec();

    graph.retain_edges(|_, edge| !top.contains(&edge.index()));

    let components = rustworkx_core::connectivity::connected_components(&graph);

    Solution(components[0].len() * components[1].len(), 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day25.txt")) == crate::Solution(54, 0));
    }
}
