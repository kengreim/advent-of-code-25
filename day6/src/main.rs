use shared::{Inputs, run_day_with_args};

type Number = i32;

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
    0
}

fn part2(input: &str) -> Number {
    0
}
