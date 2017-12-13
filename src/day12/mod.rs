use std::io::{self, BufRead};
use itertools::Itertools;
use nom::digit;
use petgraph::unionfind::UnionFind;


pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<_> = stdin.lock().lines().collect();

	// Create a union find datastructure (assumes nodes start from 0)
	let mut components = UnionFind::<u32>::new(lines.len());

	for line in lines {
		let line = line.unwrap() + "\x00";

		// Parse the line into the node and its neighbors
		let (node, neighbors): (u32, Vec<u32>) = ws!(line.as_str(),
			separated_pair!(
				map_res!(digit, str::parse),
				tag!("<->"),
				separated_list!(tag!(","), map_res!(digit, str::parse))
			)
		).unwrap().1;

		// Union nodes in the same connected component
		for neighbor in neighbors {
			components.union(node, neighbor);
		}
	}

	// Find the component label of 0
	let comp = components.find(0);
	let labeling = components.into_labeling();

	// Count nodes in this component
	let size = labeling.iter().filter(|&&c| c == comp).count();

	println!("[Part 1] Size is: {}", size);

	// Count the number of components
	let count = labeling.iter().unique().count();

	println!("[Part 2] Components: {}", count);
}
