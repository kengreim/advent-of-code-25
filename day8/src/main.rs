use petgraph::{Graph, algo};
use shared::{Inputs, run_day_with_args};
use std::collections::HashMap;

fn main() {
    run_day_with_args(
        part1,
        part2,
        Inputs {
            test: include_str!("input_test.txt"),
            full: include_str!("input.txt"),
        },
    );
}

fn part1(input: &str) -> usize {
    let boxes = parse_input(input);

    // Create a graph with every box as a node
    let mut graph = Graph::new();
    let mut nodes_lookup = HashMap::new();
    for b in &boxes {
        let new_node = graph.add_node(b);
        nodes_lookup.insert(b, new_node);
    }

    // Precalculate all distances and sort
    let mut distances = HashMap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.insert((boxes[i], boxes[j]), distance(&boxes[i], &boxes[j]));
        }
    }
    let mut distances_sorted = distances.iter().collect::<Vec<_>>();
    distances_sorted.sort_unstable_by(|((_, _), distance1), ((_, _), distance2)| {
        distance1.partial_cmp(distance2).unwrap()
    });

    // Make pairs from the 1000 closest boxes
    for ((point1, point2), _) in distances_sorted.iter().take(1000) {
        let node1 = nodes_lookup[&point1];
        let node2 = nodes_lookup[&point2];
        graph.add_edge(node1, node2, 1);
        graph.add_edge(node2, node1, 1); // Need to add nodes in both directions for the Kosaraju algo
    }

    // Find SCCs and get the biggest
    let mut components = algo::kosaraju_scc(&graph);
    components.sort_unstable_by_key(|v| usize::MAX - v.len());
    components.iter().take(3).map(|c| c.len()).product()
}

fn part2(input: &str) -> i64 {
    let boxes = parse_input(input);

    // Create a graph with every box as a node
    let mut graph = Graph::new();
    let mut nodes_lookup = HashMap::new();
    for b in &boxes {
        let new_node = graph.add_node(b);
        nodes_lookup.insert(b, new_node);
    }

    // Precalculate all distances and sort
    let mut distances = HashMap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.insert((boxes[i], boxes[j]), distance(&boxes[i], &boxes[j]));
        }
    }
    let mut distances_sorted = distances.iter().collect::<Vec<_>>();
    distances_sorted.sort_unstable_by(|((_, _), distance1), ((_, _), distance2)| {
        distance1.partial_cmp(distance2).unwrap()
    });

    // Add edges in order of closest distance and check each time how many connected components
    // there are in the entire graph
    for ((point1, point2), _) in distances_sorted {
        let node1 = nodes_lookup[&point1];
        let node2 = nodes_lookup[&point2];
        graph.add_edge(node1, node2, 1);

        if algo::connected_components(&graph) == 1 {
            return point1.x * point2.x;
        }
    }

    -1
}

type Number = i64;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: Number,
    y: Number,
    z: Number,
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut boxes = Vec::new();
    for line in input.lines() {
        let nums = line.split(',').collect::<Vec<_>>();
        boxes.push(Point {
            x: nums[0].parse::<Number>().unwrap(),
            y: nums[1].parse::<Number>().unwrap(),
            z: nums[2].parse::<Number>().unwrap(),
        });
    }
    boxes
}

fn distance(point1: &Point, point2: &Point) -> f64 {
    // Convert to f64 before squaring to avoid overflowing i32 when coords are large.
    let dx = (point2.x - point1.x) as f64;
    let dy = (point2.y - point1.y) as f64;
    let dz = (point2.z - point1.z) as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}
