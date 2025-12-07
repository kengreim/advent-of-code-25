use grid::Grid;
use petgraph::algo::all_simple_paths;
use petgraph::data::Build;
use petgraph::graph::DiGraph;
use petgraph::prelude::NodeIndex;
use petgraph::{Direction, EdgeDirection};
use shared::{Inputs, run_day_with_args};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::RandomState;
use std::iter;

type Number = i64;

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

    //beam_cols.iter().count()
    count
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

// fn part2(input: &str) -> usize {
//     let grid = parse_input(input);
//     let grid = fill_grid_with_beams(grid);
//     let splitter_rows = (0..grid.rows())
//         .rev()
//         .filter(|row_idx| {
//             for col_idx in 0..grid.cols() {
//                 if grid.get(*row_idx, col_idx).unwrap() == &'^' {
//                     return true;
//                 }
//             }
//             false
//         })
//         .collect::<Vec<_>>();
//
//     println!("{:?}", splitter_rows);
//
//     let mut sum = 0;
//     for row in splitter_rows {
//         let mut row_sum = 1;
//         for col_idx in 0..grid.cols() {
//             if *grid.get(row, col_idx).unwrap() == '|' {
//                 row_sum *= count_paths_to_cell(&grid, row, col_idx);
//             }
//         }
//         println!("{:?} {}", row, row_sum);
//         sum += row_sum;
//     }
//
//     sum
//
//     // for row in 0..grid.rows() {
//     //     for col in 0..grid.cols() {
//     //         print!("{}", grid.get(row, col).unwrap());
//     //     }
//     //     println!();
//     // }
// }

fn count_paths_to_cell(grid: &Grid<char>, row: usize, col: usize) -> usize {
    if *grid.get(row, col).unwrap() != '|' {
        panic!("Invalid row {} col {}", row, col);
    };

    let mut sum = 0;

    if col > 0
        && grid.get(row, col - 1).unwrap() == &'^'
        && grid.get(row - 1, col - 1).unwrap() == &'|'
    {
        sum += 1;
    };

    if col < grid.cols() - 1
        && grid.get(row, col + 1).unwrap() == &'^'
        && grid.get(row - 1, col + 1).unwrap() == &'|'
    {
        sum += 1;
    };

    if grid.get(row - 1, col).unwrap() == &'|' {
        sum += 1;
    }

    sum
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut grid = fill_grid_with_beams(grid);
    let mut graph = DiGraph::new();

    // We're going to be lazy and create nodes for every cell
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

    for row in 1..grid.rows() {
        for col in 0..grid.cols() {
            if grid.get(row, col).unwrap() == &'|' {
                let new_node = graph.add_node((row, col));
                all_nodes.insert((row, col), new_node);
                if col > 0 {
                    if grid.get(row, col - 1).unwrap() == &'^'
                        && grid.get(row - 1, col - 1).unwrap() == &'|'
                    {
                        let previous_node = all_nodes.get(&(row - 1, col - 1)).unwrap();
                        graph.add_edge(*previous_node, new_node, 1);
                    }
                }
                if grid.get(row - 1, col).unwrap() == &'|' {
                    let previous_node = all_nodes.get(&(row - 1, col)).unwrap();
                    graph.add_edge(*previous_node, new_node, 1);
                }
                if col < grid.cols() - 1 {
                    if grid.get(row, col + 1).unwrap() == &'^'
                        && grid.get(row - 1, col + 1).unwrap() == &'|'
                    {
                        let previous_node = all_nodes.get(&(row - 1, col + 1)).unwrap();
                        graph.add_edge(*previous_node, new_node, 1);
                    }
                }
            }
        }
    }

    let final_nodes = all_nodes.iter().filter_map(|((row, col), idx)| {
        if *row == grid.rows() - 1 {
            Some(idx)
        } else {
            None
        }
    });

    // for row in 0..grid.rows() {
    //     for col in 0..grid.cols() {
    //         print!("{}", grid.get(row, col).unwrap());
    //     }
    //     println!();
    // }
    //
    // println!(
    //     "Final nodes: {:?}",
    //     final_nodes
    //         .clone()
    //         .map(|idx| graph[*idx])
    //         .collect::<Vec<_>>()
    // );

    let mut counts: HashMap<NodeIndex, usize> = HashMap::new();
    let mut node_queue = VecDeque::from(vec![start_node.unwrap()]);
    while let Some(node) = node_queue.pop_front() {
        if let Some(count) = counts.get_mut(&node) {
            continue;
        }
        if node == start_node.unwrap() {
            counts.insert(node, 1);
        } else {
            let incoming_neighbors = graph
                .neighbors_directed(node, Direction::Incoming)
                .collect::<Vec<_>>();
            let count = incoming_neighbors.iter().map(|n| counts[n]).sum::<usize>();
            counts.insert(node, count);
        }
        let outgoing_neighbors = graph
            .neighbors_directed(node, Direction::Outgoing)
            .collect::<Vec<_>>();

        node_queue.extend(outgoing_neighbors);
    }

    final_nodes.map(|n| counts[n]).sum()
    //
    // final_nodes
    //     .map(|final_node| {
    //         all_simple_paths::<Vec<_>, _, RandomState>(
    //             &graph,
    //             *start_node.as_ref().unwrap(),
    //             *final_node,
    //             0,
    //             None,
    //         )
    //         .count()
    //     })
    //     .sum::<usize>()
}

// fn part2(input: &str) -> usize {
//     let grid = parse_input(input);
//     let mut graph = DiGraph::new();
//     let mut all_nodes = HashMap::new();
//
//     let mut current_beam_cols = HashSet::new();
//     let mut starting_node = None;
//
//     for col in 0..grid.cols() {
//         if grid.get(0, col).unwrap() == &'S' {
//             current_beam_cols.insert(col);
//             let new_node = graph.add_node((0, col));
//             all_nodes.insert((0, col), new_node);
//             starting_node = Some(new_node);
//             break;
//         }
//     }
//
//     for row in 1..grid.rows() {
//         let mut new_beam_cols = HashSet::new();
//         for beam_col in &current_beam_cols {
//             let first_node = *all_nodes
//                 .entry((row - 1, *beam_col))
//                 .or_insert_with(|| graph.add_node((row - 1, *beam_col)));
//
//             if grid.get(row, *beam_col).unwrap() == &'^' {
//                 if *beam_col > 0 {
//                     new_beam_cols.insert(beam_col - 1);
//
//                     let second_node = all_nodes
//                         .entry((row, *beam_col))
//                         .or_insert_with(|| graph.add_node((row, *beam_col - 1)));
//
//                     graph.add_edge(first_node, *second_node, 1);
//                 }
//                 if *beam_col < grid.cols() - 1 {
//                     new_beam_cols.insert(beam_col + 1);
//
//                     let second_node = all_nodes
//                         .entry((row, *beam_col))
//                         .or_insert_with(|| graph.add_node((row, *beam_col + 1)));
//
//                     graph.add_edge(first_node, *second_node, 1);
//                 }
//             } else {
//                 new_beam_cols.insert(*beam_col);
//                 let second_node = all_nodes
//                     .entry((row, *beam_col))
//                     .or_insert_with(|| graph.add_node((row, *beam_col)));
//                 graph.add_edge(first_node, *second_node, 1);
//             }
//         }
//
//         current_beam_cols = new_beam_cols;
//     }
//
//     let final_nodes = current_beam_cols
//         .iter()
//         .map(|col| all_nodes.get(&(grid.rows() - 1, *col)).unwrap())
//         .collect::<Vec<_>>();
//
//     final_nodes
//         .iter()
//         .map(|final_node| {
//             all_simple_paths::<Vec<_>, _, RandomState>(
//                 &graph,
//                 *starting_node.as_ref().unwrap(),
//                 **final_node,
//                 0,
//                 None,
//             )
//             .count()
//         })
//         .sum::<usize>()
// }

fn parse_input(input: &str) -> Grid<char> {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = lines[0].len();
    Grid::from_vec(lines.iter().flatten().copied().collect::<Vec<_>>(), cols)
}
