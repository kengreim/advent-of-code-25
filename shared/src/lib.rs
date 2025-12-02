use std::fmt::Display;
use std::time::Instant;

pub fn run_day_with_args<T1, F1, T2, F2>(part_1_fn: F1, part_2_fn: F2)
where
    T1: Display,
    T2: Display,
    F1: Fn() -> T1,
    F2: Fn() -> T2,
{
    let mut args = std::env::args();
    if let Some(part) = args.nth(1) {
        match part.as_str().to_ascii_lowercase().as_ref() {
            "part1" => {
                let start = Instant::now();
                let res = part_1_fn();
                let duration = start.elapsed();
                println!("Part 1: {res}");
                println!("{:#?}", duration);
            }
            "part2" => {
                let start = Instant::now();
                let res = part_2_fn();
                let duration = start.elapsed();
                println!("Part 2: {res}");
                println!("{:#?}", duration);
            }
            _ => eprintln!("{part} is not a valid part"),
        }
    } else {
        eprintln!("either part1 or part2 required as arg");
    }
}

// use shared::run_day_with_args;
//
// fn main() {
//     run_day_with_args(part_1, part_2);
// }
//
// fn part_1() -> &'static str {
//     "todo"
// }
//
// fn part_2() -> &'static str {
//     "todo"
// }
