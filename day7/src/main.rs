use grid::Grid;
use petgraph::Direction;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;
use shared::{Inputs, run_day_with_args};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

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
    let grid = parse_input(input);
    let mut beam_cols = HashSet::new();

    for col in 0..grid.cols() {
        if grid.get(0, col).unwrap() == &'S' {
            beam_cols.insert(col);
            break;
        }
    }

    let mut count = 0;
    for row in 1..grid.rows() {
        let mut new_beam_cols = HashSet::new();
        for beam_col in &beam_cols {
            if grid.get(row, *beam_col).unwrap() == &'^' {
                if *beam_col > 0 {
                    new_beam_cols.insert(beam_col - 1);
                }
                if *beam_col < grid.cols() - 1 {
                    new_beam_cols.insert(beam_col + 1);
                }
                count += 1;
            } else {
                new_beam_cols.insert(*beam_col);
            }
        }
        beam_cols = new_beam_cols;
    }

    count
}

fn part2(input: &str) -> usize {
    let now = Instant::now();
    let grid = parse_input(input);
    let mut grid = fill_grid_with_beams(grid);
    println!("Parse: {:?}", now.elapsed());

    let now = Instant::now();
    let mut graph = DiGraph::new();
    let mut all_nodes: HashMap<(usize, usize), NodeIndex> = HashMap::new();
    let mut start_node = None;

    for col in 0..grid.cols() {
        if grid.get(0, col).unwrap() == &'S' {
            let new_start_node = graph.add_node((0, col));
            all_nodes.insert((0, col), new_start_node);
            start_node = Some(new_start_node);
            break;
        }
    }

    let (start_row, start_col) = graph[start_node.unwrap()];
    grid[(start_row, start_col)] = '|';

    // Build an acyclic DAG from the grid
    for row in 1..grid.rows() {
        for col in 0..grid.cols() {
            if grid.get(row, col).unwrap() == &'|' {
                let new_node = graph.add_node((row, col));
                all_nodes.insert((row, col), new_node);
                if col > 0
                    && grid.get(row, col - 1).unwrap() == &'^'
                    && grid.get(row - 1, col - 1).unwrap() == &'|'
                {
                    let previous_node = all_nodes.get(&(row - 1, col - 1)).unwrap();
                    graph.add_edge(*previous_node, new_node, 1);
                }

                if grid.get(row - 1, col).unwrap() == &'|' {
                    let previous_node = all_nodes.get(&(row - 1, col)).unwrap();
                    graph.add_edge(*previous_node, new_node, 1);
                }
                if col < grid.cols() - 1
                    && grid.get(row, col + 1).unwrap() == &'^'
                    && grid.get(row - 1, col + 1).unwrap() == &'|'
                {
                    let previous_node = all_nodes.get(&(row - 1, col + 1)).unwrap();
                    graph.add_edge(*previous_node, new_node, 1);
                }
            }
        }
    }
    println!("DAG: {:?}", now.elapsed());

    // Build memoized counts from the start to the end where the count of a node is the sum
    // of the counts of incoming neighbors
    let now = Instant::now();
    let mut counts: HashMap<NodeIndex, usize> = HashMap::new();
    let mut node_queue = VecDeque::from(vec![start_node.unwrap()]);
    while let Some(node) = node_queue.pop_front() {
        if counts.contains_key(&node) {
            // We visited already
            continue;
        }
        if node == start_node.unwrap() {
            counts.insert(node, 1);
        } else {
            let count = graph
                .neighbors_directed(node, Direction::Incoming)
                .map(|n| counts[&n])
                .sum::<usize>();
            counts.insert(node, count);
        }

        // Enqueue all next neighbors so we process entire graph
        node_queue.extend(graph.neighbors_directed(node, Direction::Outgoing));
    }

    let final_nodes = all_nodes.iter().filter_map(|((row, _), idx)| {
        if *row == grid.rows() - 1 {
            Some(idx)
        } else {
            None
        }
    });

    println!("Final counts: {:?}", now.elapsed());
    final_nodes.map(|n| counts[n]).sum()
}

fn parse_input(input: &str) -> Grid<char> {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = lines[0].len();
    Grid::from_vec(lines.iter().flatten().copied().collect::<Vec<_>>(), cols)
}

fn fill_grid_with_beams(mut grid: Grid<char>) -> Grid<char> {
    let mut beam_cols = HashSet::new();

    for col in 0..grid.cols() {
        if grid.get(0, col).unwrap() == &'S' {
            beam_cols.insert(col);
            break;
        }
    }

    for row in 1..grid.rows() {
        let mut new_beam_cols = HashSet::new();
        for beam_col in &beam_cols {
            if grid.get(row, *beam_col).unwrap() == &'^' {
                if *beam_col > 0 {
                    new_beam_cols.insert(beam_col - 1);
                    grid[(row, *beam_col - 1)] = '|';
                }
                if *beam_col < grid.cols() - 1 {
                    new_beam_cols.insert(beam_col + 1);
                    grid[(row, *beam_col + 1)] = '|';
                }
            } else {
                new_beam_cols.insert(*beam_col);
                grid[(row, *beam_col)] = '|';
            }
        }
        beam_cols = new_beam_cols;
    }

    grid
}
