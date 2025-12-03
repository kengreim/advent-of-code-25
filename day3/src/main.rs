use shared::{Inputs, run_day_with_args};

fn main() {
    run_day_with_args(
        part1,
        part2_fold,
        Inputs {
            test: include_str!("input_test.txt"),
            full: include_str!("input.txt"),
        },
    );

    //test()
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        let (pos_from_end, val1) = line
            .chars()
            .rev()
            .enumerate()
            .max_by_key(|(_, c)| c.to_digit(10).unwrap())
            .unwrap();
        let pos = line.len() - 1 - pos_from_end;

        let (remainder_str, val1_first) = if pos == line.len() - 1 {
            (&line[0..line.len() - 1], false)
        } else {
            (&line[pos + 1..line.len()], true)
        };

        let val2 = remainder_str
            .chars()
            .max_by_key(|c| c.to_digit(10).unwrap())
            .unwrap()
            .to_digit(10)
            .unwrap();

        let sum = if val1_first {
            val1.to_digit(10).unwrap() * 10 + val2
        } else {
            val2 * 10 + val1.to_digit(10).unwrap()
        };

        count += sum;
    }

    count
}

fn part2(input: &str) -> u128 {
    let lines = input.lines();
    let mut sum: u128 = 0;
    for line in lines {
        let mut vals = Vec::new();
        let mut limit = 12;
        let mut current_str = line;
        let mut idx = 0;
        while limit > 0 {
            let ((max, pos), next_str) = find_leftmost_max_digit(current_str, limit);
            idx += pos;
            current_str = next_str;
            limit -= 1;
            vals.push((max, idx));
            idx += 1;
        }

        for (exp, (val, _)) in vals.iter().rev().enumerate() {
            sum += (*val as u128) * 10_u128.pow(exp as u32);
        }
    }

    sum
}

fn part2_fold(input: &str) -> u128 {
    input.lines().fold(0, |sum, line| {
        sum + (1..=12)
            .rev()
            .fold((0u128, 0usize), |(line_sum, idx), limit| {
                let (max, pos) = find_leftmost_max_digit_fold(line, idx, limit);
                (
                    line_sum + max as u128 * 10_u128.pow(limit as u32 - 1),
                    pos + 1,
                )
            })
            .0
    })
}

fn find_leftmost_max_digit_fold(line: &str, start_idx: usize, limit: usize) -> (u32, usize) {
    line[start_idx..=line.len() - limit].char_indices().fold(
        (0, 0),
        |(max, max_pos), (pos, char)| {
            if let Some(c) = char.to_digit(10)
                && c > max
            {
                (c, pos + start_idx)
            } else {
                (max, max_pos)
            }
        },
    )
}

fn find_leftmost_max_digit(line: &str, limit: usize) -> ((u32, usize), &str) {
    let shortened_by_limit = &line[0..=line.len() - limit];
    let (pos_from_end, c) = shortened_by_limit
        .chars()
        .rev()
        .enumerate()
        .max_by_key(|(_, c)| c.to_digit(10).unwrap())
        .unwrap();

    let pos = line.len() - limit - pos_from_end;

    let remainder_str = if pos == line.len() - 1 {
        &line[0..line.len() - 1]
    } else {
        &line[pos + 1..line.len()]
    };

    ((c.to_digit(10).unwrap(), pos), remainder_str)
}

// fn test()
// {
//     let line = "1113233522332432321212222313412315521343627282223261232234221222322442422324222532621323222434113195";
//     println!("{line}");
//     println!("{}", line.len());
//     let mut sum: u128 = 0;
//     let mut vals = Vec::new();
//     let mut limit = 12;
//     let mut current_str = line;
//     let mut idx = 0;
//     while limit > 0 {
//         let ((max, pos), next_str) = find_leftmost_max_digit(current_str, limit);
//         idx += pos;
//         current_str = next_str;
//         limit -= 1;
//         println!("{} {} {}", max, idx, next_str);
//         vals.push((max, idx));
//         idx += 1;
//
//         //assert_eq!(line.as_bytes()[idx].to_ascii_lowercase() as u32, max);
//     }
//
//     vals.sort_unstable_by_key(|(val, pos)| *pos);
//     println!("{vals:?}");
//
//     let mut line_sum = 0;
//     for (exp, (val, _)) in vals.iter().rev().enumerate() {
//         sum += (*val as u128) * 10_u128.pow(exp as u32);
//         line_sum += (*val as u64) * 10_u64.pow(exp as u32);
//     }
//     println!("{line_sum}");
// }
