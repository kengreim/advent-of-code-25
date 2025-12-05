use shared::{Inputs, run_day_with_args};
use std::collections::VecDeque;

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
    let (ranges, ingredients) = parse_input(input);
    let final_ranges = make_final_ranges(ranges);

    let mut count = 0;
    for n in ingredients {
        for r in &final_ranges {
            if n < r.start && n < r.end {
                break;
            } else if n == r.start {
                count += 1;
                break;
            } else if n > r.start && n <= r.end {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(input: &str) -> i64 {
    let (ranges, _) = parse_input(input);
    let final_ranges = make_final_ranges(ranges);
    final_ranges.iter().map(|r| r.end - r.start + 1).sum()
}

type Number = i64;
#[derive(Debug, Copy, Clone)]
struct Range {
    start: Number,
    end: Number,
}

impl Range {
    fn combine_with_range(&mut self, other: Range) {
        if other.start < self.start {
            self.start = other.start;
        }
        if other.end > self.end {
            self.end = other.end;
        }
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<Number>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut reading_ranges = true;
    for line in input.lines() {
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let (start, end) = line.split_once('-').unwrap();
            ranges.push(Range {
                start: start.parse::<i64>().unwrap(),
                end: end.parse::<i64>().unwrap(),
            });
        } else {
            ingredients.push(line.parse::<i64>().unwrap());
        }
    }

    (ranges, ingredients)
}

fn make_final_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

    let mut ranges_stack = VecDeque::from(ranges);
    let mut final_ranges = Vec::new();
    while !ranges_stack.is_empty() {
        let mut r1 = ranges_stack.pop_front().unwrap();
        if let Some(r2) = ranges_stack.pop_front() {
            if r1.end >= r2.start {
                r1.combine_with_range(r2);
                ranges_stack.push_front(r1);
            } else {
                final_ranges.push(r1);
                ranges_stack.push_front(r2);
            }
        } else {
            final_ranges.push(r1);
        }
    }

    final_ranges
}
