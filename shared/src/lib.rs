use std::fmt::Display;
use std::time::Instant;

pub fn run_day_with_args<T1, F1, T2, F2>(
    part_1_fn: F1,
    part_2_fn: F2,
    test_input: &str,
    full_input: &str,
) where
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
        "test" => test_input,
        "full" => full_input,
        _ => {
            eprintln!("{input_sanitized} is invalid arg for input");
            return;
        }
    };

    match part.as_str() {
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
        _ => eprintln!("{part} is not a valid part"),
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
