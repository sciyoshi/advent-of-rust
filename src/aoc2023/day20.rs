use crate::Solution;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
enum Module {
    Broadcaster,
    Sink,
    FlipFlop(bool),
    Conjunction(HashMap<NodeIndex, bool>),
}

impl Module {
    fn pulse(&mut self, pulse: bool, source: NodeIndex) -> Option<bool> {
        match self {
            Module::Broadcaster => Some(pulse),
            Module::Sink => None,
            Module::FlipFlop(state) => {
                if pulse {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            Module::Conjunction(map) => {
                *map.entry(source).or_default() = pulse;
                Some(!map.values().all(|v| *v))
            }
        }
    }
}

struct Configuration {
    graph: DiGraph<Module, ()>,
    node_indices: HashMap<String, NodeIndex>,
    targets: Vec<NodeIndex>,
}

impl Configuration {
    fn parse(input: &str) -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split(" -> ");
            let source = parts.next().unwrap();
            let to = parts.next().unwrap();

            let (module_type, name) = if source == "broadcaster" {
                (Module::Broadcaster, source)
            } else {
                match source.chars().next() {
                    Some('%') => (Module::FlipFlop(false), &source[1..]),
                    Some('&') => (Module::Conjunction(HashMap::new()), &source[1..]),
                    _ => panic!(),
                }
            };

            let from_index = *node_indices
                .entry(name.to_string())
                .or_insert_with(|| graph.add_node(Module::Sink));

            graph[from_index] = module_type;

            for to_node in to.split(", ") {
                let to_index = *node_indices
                    .entry(to_node.to_string())
                    .or_insert_with(|| graph.add_node(Module::Sink));

                graph.add_edge(from_index, to_index, ());
            }
        }

        for node in graph.node_indices() {
            let mut edges = graph
                .neighbors_directed(node, petgraph::Direction::Incoming)
                .detach();

            while let Some(neighbor) = edges.next_node(&graph) {
                if let Module::Conjunction(map) = &mut graph[node] {
                    map.entry(neighbor).or_default();
                }
            }
        }

        let check = graph
            .neighbors_directed(node_indices["rx"], petgraph::Direction::Incoming)
            .next()
            .unwrap();
        let targets = graph
            .neighbors_directed(check, petgraph::Direction::Incoming)
            .collect();

        Configuration {
            graph,
            node_indices,
            targets,
        }
    }

    fn push_button(&mut self) -> (usize, usize, Option<usize>) {
        let broadcaster = self.node_indices["broadcaster"];
        let mut queue = VecDeque::from([(broadcaster, false, broadcaster)]);
        let (mut low, mut high) = (0, 0);
        let mut triggered = None;

        while let Some((source, pulse, node)) = queue.pop_front() {
            match pulse {
                false => low += 1,
                true => high += 1,
            };

            let module = &mut self.graph[node];

            if pulse == false {
                if let Some(target) = self.targets.iter().position(|&target| target == node) {
                    triggered = Some(target);
                }
            }

            if let Some(out) = module.pulse(pulse, source) {
                for neighbor in self
                    .graph
                    .neighbors_directed(node, petgraph::Direction::Outgoing)
                {
                    queue.push_back((node, out, neighbor));
                }
            }
        }

        (low, high, triggered)
    }
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut configuration = Configuration::parse(input);

    let mut mults: Vec<Option<usize>> = vec![None; configuration.targets.len()];

    let result = (1..=1000)
        .map(|_| configuration.push_button())
        .fold((0, 0), |(low, high), (new_low, new_high, _)| {
            (low + new_low, high + new_high)
        });

    let part1 = result.0 * result.1;

    for i in 1001.. {
        if let Some(idx) = configuration.push_button().2 {
            mults[idx] = Some(i);
            if mults.iter().all(|v| v.is_some()) {
                break;
            }
        }
    }

    let part2 = crate::util::num::lcm(mults.into_iter().flatten());

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(include_str!("examples/day20.txt")) == crate::Solution(11687500, 1003002)
        );
    }
}
