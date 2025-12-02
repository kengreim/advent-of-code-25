use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let ranges = input
        .split(',')
        .map(|r| IdRange::from(r))
        .collect::<Vec<_>>();

    let mut count = 0;
    for range in ranges {
        for i in range.start..=range.end {
            let i_str = i.to_string();
            let len = i_str.len();
            if len % 2 == 0 {
                let first_half = &i_str[0..(len / 2)];
                let second_half = &i_str[(len / 2)..len];
                if first_half == second_half {
                    count += i;
                }
            }
        }
    }
    println!("Part 1 result: {}", count);
}

fn part2() {
    let start = Instant::now();
    let input = include_str!("input.txt");
    let ranges = input.split(',').map(IdRange::from);
    let mut count = 0;

    for range in ranges {
        for i in range.start..=range.end {
            let i_str = i.to_string();
            let i_len = i_str.len();
            let repeat_lengths = (1..=(i_len / 2)).filter(|j| i_len % j == 0);
            for repeat_len in repeat_lengths {
                let repeated = i_str[0..repeat_len].repeat(i_len / repeat_len);
                if repeated == i_str {
                    count += i;
                    break;
                }
            }
        }
    }
    println!("Part 2 result: {}", count);
    println!("Part 2 time: {}", start.elapsed().as_millis());
}

struct IdRange {
    pub start: i64,
    pub end: i64,
}

impl From<&str> for IdRange {
    fn from(input: &str) -> IdRange {
        let (start, end) = input.split_once("-").unwrap();
        Self {
            start: start.parse::<i64>().unwrap(),
            end: end.parse::<i64>().unwrap(),
        }
    }
}
