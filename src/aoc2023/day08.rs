// Generated with ChatGPT 4.
// https://chat.openai.com/share/f826a10a-745e-4a6b-888c-d86539a650ab

use crate::Solution;
use std::collections::HashMap;

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let mut node_mapping = HashMap::new();

    for line in lines {
        if let Some((node, children)) = line.split_once(" = ") {
            let children = children
                .trim_matches(|p| p == '(' || p == ')')
                .split(", ")
                .map(String::from)
                .collect::<Vec<_>>();
            node_mapping.insert(node.to_string(), (children[0].clone(), children[1].clone()));
        }
    }

    let part1 = navigate(&instructions, &node_mapping, "AAA".to_string());
    let part2 = lcm_paths(&instructions, &node_mapping);

    Solution(part1, part2)
}

fn navigate(
    instructions: &[char],
    node_mapping: &HashMap<String, (String, String)>,
    start: String,
) -> usize {
    let mut current_node = start;
    let mut step = 0;

    while current_node != "ZZZ" {
        let direction = instructions[step % instructions.len()];
        current_node = if direction == 'L' {
            &node_mapping[&current_node].0
        } else {
            &node_mapping[&current_node].1
        }
        .to_string();
        step += 1;
    }

    step
}

fn lcm_paths(instructions: &[char], node_mapping: &HashMap<String, (String, String)>) -> usize {
    let starting_nodes = node_mapping
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
    let mut lcm = 1;

    for node in starting_nodes {
        let (cycle_length, _) = find_cycle(instructions, node_mapping, node);
        lcm = lcm.lcm(&cycle_length);
    }

    lcm
}

fn find_cycle(
    instructions: &[char],
    node_mapping: &HashMap<String, (String, String)>,
    start: String,
) -> (usize, Vec<usize>) {
    let mut visited = HashMap::new();
    let mut current_node = start;
    let mut instruction_position = 0;
    let mut step = 0;
    let mut z_offsets = Vec::new();

    while !visited.contains_key(&(current_node.clone(), instruction_position)) {
        if current_node.ends_with('Z') {
            z_offsets.push(step);
        }

        visited.insert((current_node.clone(), instruction_position), step);
        let direction = instructions[instruction_position];
        current_node = if direction == 'L' {
            &node_mapping[&current_node].0
        } else {
            &node_mapping[&current_node].1
        }
        .to_string();
        instruction_position = (instruction_position + 1) % instructions.len();
        step += 1;
    }

    let cycle_start_step = visited[&(current_node, instruction_position)];
    let cycle_length = step - cycle_start_step;

    z_offsets.retain(|&offset| offset >= cycle_start_step && offset < step);
    (cycle_length, z_offsets)
}

impl Lcm for usize {
    fn lcm(&self, other: &Self) -> Self {
        self * other / gcd(*self, *other)
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

trait Lcm {
    fn lcm(&self, other: &Self) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(
            super::solve(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ) == crate::Solution(6, 3)
        );
    }
}
