use crate::Solution;
use petgraph::{algo::dijkstra, graph::NodeIndex, Graph};

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut graph: Graph<u8, ()> = Graph::default();

    let mut start = NodeIndex::new(0);
    let mut end = NodeIndex::new(0);

    let lines: Vec<_> = input.lines().collect();

    let width = lines[0].len() as isize;

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let elevation = match c {
                'S' => 0,
                'E' => 25,
                _ => (c as u8) - ('a' as u8),
            };
            let node = graph.add_node(elevation);
            if c == 'S' {
                start = node;
            } else if c == 'E' {
                end = node;
            }
            if j > 0 {
                let left = NodeIndex::new(node.index() - 1);
                if *graph.node_weight(left).unwrap() <= elevation + 1 {
                    graph.add_edge(NodeIndex::new(node.index() - 1), node, ());
                }
                if elevation <= *graph.node_weight(left).unwrap() + 1 {
                    graph.add_edge(node, NodeIndex::new(node.index() - 1), ());
                }
            }
            if i > 0 {
                let up = NodeIndex::new(node.index() - width as usize);
                if *graph.node_weight(up).unwrap() <= elevation + 1 {
                    graph.add_edge(up, node, ());
                }
                if elevation <= *graph.node_weight(up).unwrap() + 1 {
                    graph.add_edge(node, up, ());
                }
            }
        }
    }

    let result = dijkstra(&graph, end, None, |_| 1);

    let part1 = result[&start];

    let part2 = *graph
        .node_indices()
        .filter(|&i| *graph.node_weight(i).unwrap() == 0)
        .filter_map(|start| result.get(&start))
        .min()
        .unwrap();

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day12.txt")) == crate::Solution(31, 29));
    }
}
