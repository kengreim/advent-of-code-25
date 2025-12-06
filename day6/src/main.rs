use crate::Operator::{Add, Multiply};
use grid::Grid;
use shared::{Inputs, run_day_with_args};
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
fn part1(input: &str) -> Number {
    let problems = parse_input(input);
    problems.iter().fold(0, |acc, (vals, operator)| {
        acc + match operator {
            Add => vals.iter().sum::<Number>(),
            Multiply => vals.iter().product::<Number>(),
        }
    })
}

fn part2(input: &str) -> Number {
    let problems = parse_input_2(input);

    let mut sum = 0;

    for (vals, operator) in problems {
        let cols = vals[0].len();
        let problem_grid = Grid::from_vec(vals.iter().flatten().collect::<Vec<_>>(), cols);
        let mut new_vals = Vec::new();
        for col in 0..cols {
            let mut col_val = String::new();
            for row in 0..problem_grid.rows() {
                col_val.push(**problem_grid.get(row, col).unwrap());
            }
            new_vals.push(col_val.trim().parse::<Number>().unwrap());
        }

        match operator {
            Add => sum += new_vals.iter().sum::<Number>(),
            Multiply => sum += new_vals.iter().product::<Number>(),
        }
    }

    sum
}

fn parse_input(input: &str) -> Vec<(Vec<Number>, Operator)> {
    //Vec<(Number, Operator)> {
    let splits = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let num_cols = splits[0].len();
    let mut problems = Vec::new();
    for col in 0..num_cols {
        let mut vals = Vec::new();
        for row in splits[0..splits.len() - 1].iter() {
            let val = row[col].parse::<Number>().unwrap();
            vals.push(val);
        }
        let operator = if splits[splits.len() - 1][col] == "*" {
            Multiply
        } else {
            Add
        };
        problems.push((vals, operator));
    }

    problems
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn parse_input_2(input: &str) -> Vec<(Vec<Vec<char>>, Operator)> {
    let lines = input.lines().collect::<Vec<_>>();
    let max_cols = lines.iter().map(|l| l.chars().count()).max().unwrap();

    let padded_lines = lines
        .into_iter()
        .map(|l| {
            let mut chars = l.chars().collect::<Vec<_>>();
            let additional = iter::repeat(' ')
                .take(max_cols - chars.len())
                .collect::<Vec<_>>();
            chars.extend(additional);
            chars
        })
        .collect::<Vec<_>>();

    let grid = Grid::from_vec(
        padded_lines.into_iter().flatten().collect::<Vec<_>>(),
        max_cols,
    );

    let mut blank_cols = Vec::new();
    for col in 0..grid.cols() {
        for row in 0..grid.rows() {
            if *grid.get(row, col).unwrap() != ' ' {
                break;
            }
            if row == grid.rows() - 1 {
                blank_cols.push(col);
            }
        }
    }
    blank_cols.push(grid.cols());

    let mut current_col = 0;
    let mut parsed = Vec::new();
    for separator_col in blank_cols {
        let mut vals = Vec::new();
        for row in 0..grid.rows() {
            let chars_vec = (current_col..separator_col).fold(Vec::new(), |mut s, col| {
                s.push(*grid.get(row, col).unwrap());
                s
            });
            if row < grid.rows() - 1 {
                vals.push(chars_vec);
            } else {
                if chars_vec.iter().collect::<String>().trim() == "*" {
                    parsed.push((vals.clone(), Multiply));
                } else {
                    parsed.push((vals.clone(), Add));
                }
            }
        }
        current_col = separator_col + 1;
    }
    
    parsed
}
