use crate::Solution;
use crate::util::euclid::Vec3Ext;
use euclid::default::{Point2D, Point3D, Vector3D};
use itertools::Itertools;
use petgraph::{
    Graph,
    graph::NodeIndex,
    visit::{VisitMap, Visitable},
};
use std::collections::{HashMap, HashSet, VecDeque};

fn parse_point(input: &str) -> Point3D<isize> {
    Point3D::from(
        input
            .split(",")
            .flat_map(str::parse)
            .collect_tuple::<(isize, isize, isize)>()
            .unwrap(),
    )
}

#[derive(Debug)]
struct Brick {
    start: Point3D<isize>,
    size: Vector3D<isize>,
}

impl Brick {
    fn parse(input: &str) -> Self {
        let mut parts = input.split("~");
        let start = parse_point(parts.next().unwrap());
        let end = parse_point(parts.next().unwrap());
        let size = end - start;
        Brick { start, size }
    }

    fn height(&self) -> isize {
        self.size.z + 1
    }

    fn cover(&self) -> Vec<Point2D<isize>> {
        if self.size.x == 0 && self.size.y == 0 {
            vec![self.start.xy()]
        } else {
            (0..=self.size.norm1())
                .map(|i| self.start.xy() + self.size.xy() / self.size.norm1() * i)
                .collect()
        }
    }
}

fn fall_count(graph: &Graph<usize, ()>, start: NodeIndex) -> usize {
    let mut stack: VecDeque<NodeIndex> = VecDeque::from([start]);
    let mut visit_map = graph.visit_map();
    let mut total = 0;
    visit_map.visit(start);

    // breadth-first traversal
    while let Some(node) = stack.pop_front() {
        total += 1;
        for succ in graph.neighbors(node) {
            if graph
                .neighbors_directed(succ, petgraph::Direction::Incoming)
                .all(|pred| visit_map.is_visited(&pred))
                && visit_map.visit(succ)
            {
                stack.push_back(succ);
            }
        }
    }

    total - 1
}

pub fn solve(input: &str) -> Solution<usize, usize> {
    let mut bricks: Vec<Brick> = input.lines().map(Brick::parse).collect();

    let mut heights: HashMap<Point2D<isize>, (isize, usize)> = HashMap::new();
    let mut edges: HashSet<(u32, u32)> = HashSet::default();

    bricks.sort_by_key(|brick| brick.start.z);

    for (i, brick) in bricks.iter_mut().enumerate() {
        let cover = brick.cover();
        let new_height = cover
            .iter()
            .map(|xy| heights.get(xy).unwrap_or(&(0, 0)).0)
            .max()
            .unwrap_or(0)
            + 1;

        for xy in cover {
            if let Some(&(h, j)) = heights.get(&xy)
                && h == new_height - 1
            {
                edges.insert((j as u32, i as u32));
            }

            heights.insert(xy, (new_height + brick.height() - 1, i));
        }

        brick.start.z = new_height;
    }

    let graph = Graph::<usize, ()>::from_edges(&edges);

    let mut part1 = 0;
    let mut part2 = 0;

    for node in graph.node_indices() {
        if graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
            .all(|n| {
                graph
                    .neighbors_directed(n, petgraph::Direction::Incoming)
                    .count()
                    > 1
            })
        {
            part1 += 1;
        } else {
            part2 += fall_count(&graph, node);
        }
    }

    Solution(part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert!(super::solve(include_str!("examples/day22.txt")) == crate::Solution(5, 7));
    }
}
