use geo::{Covers, LineString, Polygon, Rect};
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

fn part1(input: &str) -> Number {
    let points = parse_input(input);
    (0..points.len() - 1)
        .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            let p1 = points[i];
            let p2 = points[j];
            ((p2.col - p1.col).abs() + 1 as Number) * ((p2.row - p1.row).abs() + 1 as Number)
        })
        .max()
        .unwrap()

    // let points = parse_input(input);
    // let distinct_pairs = (0..points.len() - 1)
    //     .flat_map(|i| {
    //         (i..points.len())
    //             .map(|j| (points[i], points[j]))
    //             .collect::<Vec<_>>()
    //     })
    //     .collect::<Vec<_>>();
    //
    // distinct_pairs
    //     .iter()
    //     .map(|(p1, p2)| ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1))
    //     .max()
    //     .unwrap()
}

fn part2(input: &str) -> f64 {
    let mut points = parse_input(input);
    points.push(points[0].clone());

    let poly: Polygon<f64> = Polygon::new(
        LineString::from(
            points
                .iter()
                .map(|Point { col, row }| (*col as f64, *row as f64))
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    // Skip the last index since we duplicated that from the front already
    let all_red_pair_indices =
        (0..points.len() - 2).flat_map(|i| (i + 1..points.len() - 1).map(move |j| (i, j)));

    let mut max = 0f64;
    for (i, j) in all_red_pair_indices {
        let p1 = points[i];
        let p2 = points[j];
        let rect = Rect::new(
            geo::Coord {
                x: p1.col as f64,
                y: p1.row as f64,
            },
            geo::Coord {
                x: p2.col as f64,
                y: p2.row as f64,
            },
        );
        if poly.covers(&rect) {
            max = max.max(
                ((p1.row as f64 - p2.row as f64).abs() + 1f64)
                    * ((p1.col as f64 - p2.col as f64).abs() + 1f64),
            );
        }
    }

    max
}

type Number = i64;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point {
    col: Number,
    row: Number,
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point {
                col: x.parse::<Number>().unwrap(),
                row: y.parse::<Number>().unwrap(),
            }
        })
        .collect()
}
