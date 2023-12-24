// Partially generated with ChatGPT
// https://chat.openai.com/share/27ff0881-a32a-4fb9-adb4-5a8948415e4b
// https://chat.openai.com/share/beb0c8c4-a10d-47de-822e-4f08b42fc710

use crate::Solution;
use fixedbitset::FixedBitSet;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{EdgeRef, IntoNodeReferences, NodeRef};

fn create_graph(input: &str, slopes: bool) -> (Graph<(usize, usize), usize>, NodeIndex, NodeIndex) {
    let mut graph = Graph::<(usize, usize), usize>::new();
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut nodes = vec![vec![None; width]; height];

    // Create nodes for '.' and slope symbols
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            match lines[y].as_bytes()[x] as char {
                '.' | '^' | '>' | 'v' | '<' => {
                    nodes[y][x] = Some(graph.add_node((x, y)));
                }
                _ => {}
            }
        }
    }

    // Create edges based on neighbors and slope directions
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if let Some(node) = nodes[y][x] {
                // Add edges to adjacent nodes for '.' cells
                if lines[y].as_bytes()[x] as char == '.' || slopes {
                    if let Some(target_node) = nodes[y][x - 1] {
                        graph.add_edge(node, target_node, 1);
                    }
                    if let Some(target_node) = nodes[y][x + 1] {
                        graph.add_edge(node, target_node, 1);
                    }
                    if let Some(target_node) = nodes[y - 1][x] {
                        graph.add_edge(node, target_node, 1);
                    }
                    if let Some(target_node) = nodes[y + 1][x] {
                        graph.add_edge(node, target_node, 1);
                    }
                } else if !slopes {
                    // Add directed edge for slope symbols
                    let next_node = match lines[y].as_bytes()[x] as char {
                        '^' => nodes[y - 1][x],
                        '>' => nodes[y][x + 1],
                        'v' => nodes[y + 1][x],
                        '<' => nodes[y][x - 1],
                        _ => None,
                    };

                    if let Some(target_node) = next_node {
                        graph.add_edge(node, target_node, 1);
                    }
                }
            }
        }
    }

    let mut start = nodes[1][1].unwrap();
    let mut end = nodes[height - 2][width - 2].unwrap();

    if slopes {
        collapse(&mut graph);

        for node in graph.node_references() {
            if *node.weight() == (1, 1) {
                start = node.id();
            } else if *node.weight() == (width - 2, height - 2) {
                end = node.id();
            }
        }
    }

    (graph, start, end)
}

fn collapse(graph: &mut Graph<(usize, usize), usize>) {
    let mut removed = true;

    while removed {
        removed = false;
        for node in graph.node_indices() {
            if graph.neighbors(node).count() == 2 {
                let mut neighbors = graph.edges(node);
                let edge1 = neighbors.next().unwrap();
                let nb1 = edge1.target();
                let edge2 = neighbors.next().unwrap();
                let nb2 = edge2.target();
                let weight = edge1.weight() + edge2.weight();
                graph.add_edge(nb1, nb2, weight);
                graph.add_edge(nb2, nb1, weight);
                graph.remove_node(node);
                removed = true;
            }
        }
    }
}

// Find the total weight of the edges in the longest path from start to end
fn longest_path(graph: &Graph<(usize, usize), usize>, start: NodeIndex, end: NodeIndex) -> usize {
    let mut stack = vec![(start, 0, false)];
    let mut max_path_weight = 0;
    let mut visited = FixedBitSet::with_capacity(graph.node_count());

    while let Some((node, path_weight, is_backtracking)) = stack.pop() {
        if is_backtracking {
            // We are backtracking, unmark the node as visited
            visited.set(node.index(), false);
            continue;
        }

        // If the end node is reached, update the longest path
        if node == end {
            max_path_weight = max_path_weight.max(path_weight);
            continue;
        }

        if visited.contains(node.index()) {
            // Skip if the node was already visited
            continue;
        }

        // Mark the node as visited
        visited.insert(node.index());

        // Mark that we are starting to backtrack
        stack.push((node, path_weight, true));

        // Explore neighbors
        for edge in graph.edges(node) {
            let next_node = edge.target();
            if !visited.contains(next_node.index()) {
                let edge_weight = *edge.weight();
                stack.push((next_node, path_weight + edge_weight, false));
            }
        }
    }

    max_path_weight + 2
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let (graph, start, end) = create_graph(input, false);

    let part1 = longest_path(&graph, start, end);

    let (graph, start, end) = create_graph(input, true);

    let part2 = longest_path(&graph, start, end);

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day23.txt")) == crate::Solution(94, 154));
    }
}
