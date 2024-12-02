use std::collections::HashMap;

use crate::Solution;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    multi::separated_list1,
};
use petgraph::Graph;

fn parse_valve(input: &str) -> IResult<&str, (&str, u64, Vec<&str>)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow) = u64(input)?;
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(input)?;
    let (input, tunnels) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((input, (valve, flow, tunnels)))
}

// fn find_paths<'a>(
//     graph: &'a Graph<(&str, u64), ()>,
//     time: u64,
//     value: u64,
//     path: Vec<NodeIndex>,
// ) -> Box<dyn Iterator<Item = u64> + 'a> {
//     if time == 0 {
//         return Box::new(std::iter::once(value));
//     }

//     let node = path.last().unwrap();
//     let flow = graph[*node].1;

//     if flow != 0 && time >= 2 {
//         let path = path.clone();
//         return Box::new(
//             graph
//                 .neighbors(*node)
//                 .filter_map(move |tunnel| {
//                     if path.contains(&tunnel) {
//                         None
//                     } else {
//                         let mut path = path.clone();
//                         path.push(tunnel);
//                         Some(find_paths(graph, time - 2, value + (time - 1) * flow, path))
//                     }
//                 })
//                 .flatten(),
//         );
//     }

//     let path = path.clone();
//     Box::new(
//         graph
//             .neighbors(*node)
//             .filter_map(move |tunnel| {
//                 if path.contains(&tunnel) {
//                     None
//                 } else {
//                     let mut path = path.clone();
//                     path.push(tunnel);
//                     Some(find_paths(graph, time - 1, value, path))
//                 }
//             })
//             .flatten(),
//     )
// }

// fn find_elephant_paths<'a>(
//     graph: &'a Graph<(&str, u64), ()>,
//     time: u64,
//     value: u64,
//     path: Vec<NodeIndex>,
//     elephant: Vec<NodeIndex>,
// ) -> Box<dyn Iterator<Item = u64> + 'a> {
//     if time == 0 {
//         return Box::new(std::iter::once(value));
//     }

//     let node = path.last().unwrap();
//     let elephant = path.last().unwrap();
//     let flow = graph[*node].1;

//     if flow != 0 && time >= 2 {
//         let path = path.clone();
//         return Box::new(
//             graph
//                 .neighbors(*node)
//                 .filter_map(move |tunnel| {
//                     if path.contains(&tunnel) {
//                         None
//                     } else {
//                         let mut path = path.clone();
//                         path.push(tunnel);
//                         Some(find_paths(graph, time - 2, value + (time - 1) * flow, path))
//                     }
//                 })
//                 .flatten(),
//         );
//     }

//     let path = path.clone();
//     Box::new(
//         graph
//             .neighbors(*node)
//             .filter_map(move |tunnel| {
//                 if path.contains(&tunnel) {
//                     None
//                 } else {
//                     let mut path = path.clone();
//                     path.push(tunnel);
//                     Some(find_paths(graph, time - 1, value, path))
//                 }
//             })
//             .flatten(),
//     )
// }

pub fn solve(input: &str) -> Solution<isize, isize> {
    let valves: Vec<_> = input.lines().map(|l| parse_valve(l).unwrap().1).collect();
    let mut graph: Graph<(&str, u64), ()> = Graph::new();
    let nodes: HashMap<&str, _> = valves
        .iter()
        .map(|valve| (valve.0, graph.add_node((valve.0, valve.1))))
        .collect();

    graph.extend_with_edges(
        valves
            .iter()
            .flat_map(|v| v.2.iter().map(|e| (nodes[v.0], nodes[e]))),
    );

    let paths = petgraph::algo::floyd_warshall(&graph, |_| 1);

    println!("{:?}", paths);

    // let paths: Vec<_> = find_paths(&graph, 30, 0, vec![nodes["AA"]]).collect();

    // println!("{:?}", paths.iter().max().unwrap());

    Solution(0, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve("") == crate::Solution(0, 0));
    }
}
