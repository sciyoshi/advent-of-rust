use crate::Solution;
use petgraph::graphmap::UnGraphMap;

pub fn solve(input: &str) -> Solution<u32, usize> {
    let edges: Vec<_> = input.lines().collect();

    let graph = UnGraphMap::<_, ()>::from_edges(edges.iter().map(|l| {
        let edge: Vec<_> = l.split(')').collect();

        (edge[0], edge[1])
    }));

    let mut distances = Vec::new();
    let mut seen = Vec::new();
    let mut stack = vec![("COM", 0)];

    // Perform a depth-first search to calculate the distance to the root node.
    while let Some((node, distance)) = stack.pop() {
        if seen.contains(&node) {
            continue;
        }

        seen.push(node);
        distances.push(distance);

        for next in graph.neighbors(node) {
            stack.push((next, distance + 1));
        }
    }

    // Use the astar() function to find the shortest path between the nodes labeled "YOU" and "SAN".
    let path = petgraph::algo::astar(&graph, "YOU", |node| node == "SAN", |_| 1, |_| 0);

    let part1 = distances.into_iter().sum::<u32>();
    let part2 = path.map(|(_, path)| path.len()).unwrap() - 3;

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN").1
                == 4
        );
    }
}
