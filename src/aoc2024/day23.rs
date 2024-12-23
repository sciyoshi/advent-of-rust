use std::collections::HashSet;

use crate::Solution;
use itertools::Itertools;
use petgraph::{
    graphmap::UnGraphMap,
    visit::{GetAdjacencyMatrix, IntoNodeIdentifiers},
};

// algorithm BronKerbosch2(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     choose a pivot vertex u in P ⋃ X
//     for each vertex v in P \ N(u) do
//         BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}

fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + petgraph::visit::IntoNeighbors,
    G::NodeId: Eq + std::hash::Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .cloned()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = HashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

pub fn solve(input: &str) -> Solution<usize, String> {
    let graph: UnGraphMap<[char; 2], ()> = UnGraphMap::from_edges(input.lines().map(|line| {
        let chars = line.chars().collect_vec();
        ([chars[0], chars[1]], [chars[3], chars[4]])
    }));

    let mut part1 = 0;
    for node in graph.nodes() {
        for nb in graph.neighbors(node) {
            for nb2 in graph.neighbors(nb) {
                if node != nb2
                    && graph.contains_edge(node, nb2)
                    && node < nb
                    && nb < nb2
                    && (node[0] == 't' || nb[0] == 't' || nb2[0] == 't')
                {
                    part1 += 1;
                }
            }
        }
    }

    let r = HashSet::new();
    let p = graph.node_identifiers().collect::<HashSet<[char; 2]>>();
    let x = HashSet::new();
    let result = bron_kerbosch_pivot(&graph, &(), r, p, x);
    let max_clique = result.iter().max_by_key(|clique| clique.len()).unwrap();
    let part2 = max_clique
        .iter()
        .sorted()
        .map(|el| format!("{}{}", el[0], el[1]))
        .join(",");

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day23.txt"))
                == crate::Solution(7, "co,de,ka,ta".to_string())
        );
    }
}
