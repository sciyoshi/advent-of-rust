use crate::Solution;
use itertools::Itertools;
use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, u32},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
};
use petgraph::unionfind::UnionFind;

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<u32>)> {
    separated_pair(
        u32,
        delimited(space1, tag("<->"), space1),
        separated_list1(terminated(tag(","), space1), u32),
    )(input)
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let lines: Vec<_> = input.lines().collect();

    // Create a union find datastructure (assumes nodes start from 0)
    let mut components = UnionFind::<u32>::new(lines.len());

    for line in lines {
        let line = line.to_string() + "\x00";

        // Parse the line into the node and its neighbors
        let (node, neighbors): (u32, Vec<u32>) = parse_line(line.as_str()).unwrap().1;

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

    // Count the number of components
    let count = labeling.iter().unique().count();

    Solution(size, count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day12.txt")) == crate::Solution(6, 2));
    }
}
