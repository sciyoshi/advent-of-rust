use std::collections::HashMap;
use petgraph::Graph;
use regex::Regex;
use std::io::{self, BufRead};

struct Node {
	weight: u32,
	total: u32
}

pub fn solve() {
	// Read stdin into an array of memory bank values
	let stdin = io::stdin();
	let re = Regex::new("[[:word:]]+").unwrap();

	let lines: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|l| l.ok())
		.map(|l| re.find_iter(&l)
			.map(|m| m.as_str())
			.collect())
		.collect();

	let mut indices = HashMap::new();
	let mut graph = Graph::<String, ()>::new();

	for line in lines {
		let idx = graph.add_node(line[0].to_string());
		indices.insert(line[0].to_string(), idx);
	}

	for line in lines {
		for child in &line[2..] {
			graph.add_edge(indices[line[0]], indices[child], ());
			println!("{:?}", child);
		}
	}
}