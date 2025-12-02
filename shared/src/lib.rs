use std::fmt::Display;
use std::time::Instant;

pub struct Inputs<'a> {
    pub test: &'a str,
    pub full: &'a str,
}

pub fn run_day_with_args<T1, F1, T2, F2>(part_1_fn: F1, part_2_fn: F2, inputs: Inputs<'_>)
where
    T1: Display,
    T2: Display,
    F1: Fn(&str) -> T1,
    F2: Fn(&str) -> T2,
{
    let args = std::env::args().collect::<Vec<_>>();
    let Some(part) = args.get(1) else {
        eprintln!("missing arg for part");
        return;
    };

    let Some(input) = args.get(2) else {
        eprintln!("missing arg for input");
        return;
    };

    let input_sanitized = input.to_ascii_lowercase();
    let input = match input_sanitized.as_str() {
        "test" => inputs.test,
        "full" => inputs.full,
        _ => {
            eprintln!("{input_sanitized} is not a valid arg for input");
            return;
        }
    };

    match part.to_ascii_lowercase().as_str() {
        "part1" => {
            let start = Instant::now();
            let res = part_1_fn(input);
            let duration = start.elapsed();
            println!("Part 1: {res}");
            println!("{:#?}", duration);
        }
        "part2" => {
            let start = Instant::now();
            let res = part_2_fn(input);
            let duration = start.elapsed();
            println!("Part 2: {res}");
            println!("{:#?}", duration);
        }
        "both" => {
            let start = Instant::now();
            let res = part_1_fn(input);
            let duration = start.elapsed();
            println!("Part 1: {res}");
            println!("{:#?}", duration);
            let start = Instant::now();
            let res = part_2_fn(input);
            let duration = start.elapsed();
            println!("Part 2: {res}");
            println!("{:#?}", duration);
        }
        _ => eprintln!("{part} is not a valid arg for part"),
    }
}

// use shared::run_day_with_args;
//
// fn main() {
//     run_day_with_args(
//         part1,
//         part2,
//         include_str!("input.txt"),
//         include_str!("input_test.txt"),
//     );
// }
//
// fn part1(input: &str) -> &'static str {
//     "todo"
// }
//
// fn part2(input: &str) -> &'static str {
//     "todo"
// }
