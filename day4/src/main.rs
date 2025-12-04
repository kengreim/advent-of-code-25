use grid::Grid;
use shared::{Inputs, run_day_with_args};

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

fn part1(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = lines[0].len();
    let grid = Grid::from_vec(lines.into_iter().flatten().collect(), cols);
    grid.indexed_iter()
        .map(|((row, col), char)| {
            (*char == '@' && count_adjacent_with_fn(&grid, row, col, |char2| char2 == '@') < 4)
                as i32
        })
        .sum::<i32>()
}

fn part2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cols = lines[0].len();
    let mut grid = Grid::from_vec(lines.into_iter().flatten().collect(), cols);

    let mut start_count = 0;
    loop {
        let mut new_grid = grid.clone();
        let removable = grid
            .indexed_iter()
            .filter(|((row, col), char)| {
                **char == '@' && count_adjacent_with_fn(&grid, *row, *col, |char2| char2 == '@') < 4
            })
            .collect::<Vec<_>>();

        if removable.is_empty() {
            break;
        }
        start_count += removable.len();
        for ((row, col), _) in removable {
            let cell_ref = new_grid.get_mut(row, col).unwrap();
            *cell_ref = 'x'
        }
        grid = new_grid;
    }

    start_count
}

fn count_adjacent_with_fn<T, F>(grid: &Grid<T>, row: usize, col: usize, func: F) -> usize
where
    F: Fn(T) -> bool,
    T: Copy,
{
    const OFFSET_MATRIX: [(i32, i32); 8] = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    OFFSET_MATRIX
        .iter()
        .filter(|(row_offset, col_offset)| {
            if let Some(adjacent_row) = safe_offset(row, *row_offset)
                && let Some(adjacent_col) = safe_offset(col, *col_offset)
                && let Some(val) = grid.get(adjacent_row, adjacent_col)
            {
                func(*val)
            } else {
                false
            }
        })
        .count()
}

fn safe_offset(val: usize, offset: i32) -> Option<usize> {
    if offset < 0 {
        val.checked_sub((offset).abs() as u32 as usize)
    } else {
        Some(val + offset as usize)
    }
}
