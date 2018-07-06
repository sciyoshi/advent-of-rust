use std::io::{self, BufRead};
use nom::*;
use crate::util::num;
use petgraph::{Graph, Undirected, visit::{EdgeRef, NodeIndexable}, graph::NodeIndex};
use bit_set::BitSet;

#[derive(Default)]
struct Stats {
	strongest: u32,
	longest: u32,
	length: usize
}


fn visit(graph: &Graph<u32, u32, Undirected>, node: NodeIndex<u32>, visited: &mut BitSet, weight: u32, acc: &mut Stats) {
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

crate fn solve() {
	let stdin = io::stdin();

	let edges: Vec<(u32, u32, u32)> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.filter_map(|line| separated_pair!(line.as_str(),
			call!(num), tag_s!("/"), call!(num)).to_result().ok())
		.map(|(left, right)| (left as u32, right as u32, (left + right) as u32))
		.collect();

	let graph = Graph::<u32, u32, Undirected>::from_edges(&edges);
	let mut visited = BitSet::with_capacity(graph.edge_count());
	let mut stats = Stats::default();

	visit(&graph, graph.from_index(0), &mut visited, 0, &mut stats);

	println!("[Part 1] Strongest bridge: {}", stats.strongest);
	println!("[Part 2] Strength of longest bridge: {}", stats.longest);
}
