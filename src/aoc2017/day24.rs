use crate::utils::extract_integers;
use crate::Solution;
use bit_set::BitSet;
use petgraph::{
    graph::NodeIndex,
    visit::{EdgeRef, NodeIndexable},
    Graph, Undirected,
};

#[derive(Default)]
struct Stats {
    strongest: u32,
    longest: u32,
    length: usize,
}

fn visit(
    graph: &Graph<u32, u32, Undirected>,
    node: NodeIndex<u32>,
    visited: &mut BitSet,
    weight: u32,
    acc: &mut Stats,
) {
    if weight > acc.strongest {
        acc.strongest = weight;
    }

    if visited.len() > acc.length || visited.len() == acc.length && weight > acc.longest {
        acc.length = visited.len();
        acc.longest = weight;
    }

    for edge in graph.edges(node) {
        let index = edge.id().index();

        if !visited.contains(index) {
            visited.insert(index);

            visit(graph, edge.target(), visited, weight + edge.weight(), acc);

            visited.remove(index);
        }
    }
}

pub fn solve(input: &str) -> Solution<u32, u32> {
    let edges: Vec<(u32, u32, u32)> = input
        .lines()
        .map(|line| {
            let ints = extract_integers(line);
            (ints[0], ints[1])
        })
        .map(|(left, right)| (left, right, left + right))
        .collect();

    let graph = Graph::<u32, u32, Undirected>::from_edges(&edges);
    let mut visited = BitSet::with_capacity(graph.edge_count());
    let mut stats = Stats::default();

    visit(&graph, graph.from_index(0), &mut visited, 0, &mut stats);

    Solution(stats.strongest, stats.longest)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10") == crate::Solution(31, 19)
        );
    }
}
