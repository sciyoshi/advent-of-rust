use petgraph::graphmap::UnGraphMap;
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn paths<'a>(
    graph: &UnGraphMap<&'a str, ()>,
    node: &'a str,
    visited: &HashSet<&'a str>,
    twice: bool,
) -> u32 {
    if node == "end" {
        return 1;
    }

    let mut visited = visited.clone();
    visited.insert(node);

    let mut total = 0;

    for next in graph.neighbors(node) {
        if next.to_ascii_lowercase() == next && visited.contains(&next) {
            if next != "start" && !twice {
                total += paths(graph, next, &visited, true);
            }
            continue;
        }
        total += paths(graph, next, &visited, twice);
    }

    total
}

pub fn solve() {
    let data: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut graph: UnGraphMap<&str, ()> = UnGraphMap::new();

    for line in &data {
        let mut split = line.split("-");
        let n1 = split.next().unwrap();
        let n2 = split.next().unwrap();
        graph.add_edge(n1, n2, ());
    }

    println!(
        "[Part 1] {:?}",
        paths(&graph, "start", &mut HashSet::new(), true)
    );
    println!(
        "[Part 2] {:?}",
        paths(&graph, "start", &mut HashSet::new(), false)
    );
}
