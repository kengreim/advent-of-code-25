use crate::Direction::{Left, Right};
use shared::run_day_with_args;

fn main() {
    let full_input = include_str!("input.txt");
    let test_input = include_str!("input_test.txt");
    run_day_with_args(part1, part2, test_input, full_input);
}

fn part1(input: &str) -> i32 {
    let mut position = 50;
    let mut num_0 = 0;
    for line in input.lines() {
        let num = line[1..].parse::<i32>().unwrap();
        if line.starts_with('R') {
            position = (position + num) % 100;
        } else {
            position = if (position - num) > 0 {
                position - num
            } else {
                (100 - (((position - num) * -1) % 100)) % 100
            }
        }

        if position == 0 {
            num_0 += 1;
        }
    }

    num_0
}

fn part2(input: &str)  -> i32 {
    let mut position = 50;
    let mut num_0_clicks = 0;
    for line in input.lines() {
        let num = line[1..].parse::<i32>().unwrap();
        if line.starts_with('R') {
            num_0_clicks += (position + num) / 100;
            position = (position + num) % 100;
        } else {
            position = if (position - num) > 0 {
                position - num
            } else {
                if position - num == 0 {
                    num_0_clicks += 1;
                } else {
                    let extra_click = if position > 0 { 1 } else { 0 };
                    num_0_clicks += ((position - num) * -1) / 100 + extra_click;
                }

                (100 - (((position - num) * -1) % 100)) % 100
            }
        }
    }

    num_0_clicks
}

fn part2_bruteforce() -> i32 {
    let input = include_str!("input.txt");
    let mut position = 50;
    let mut num_0_clicks = 0;
    for line in input.lines() {
        let num = line[1..].parse::<i32>().unwrap();
        let direction = if line.starts_with('R') { Right } else { Left };
        for _ in 0..num {
            position = turn_dial_once(position, direction);
            if position == 0 {
                num_0_clicks += 1
            }
        }
    }
    num_0_clicks
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

fn turn_dial_once(position: i32, direction: Direction) -> i32 {
    match direction {
        Left => {
            if position == 0 {
                99
            } else {
                position - 1
            }
        }
        Right => {
            if position == 99 {
                0
            } else {
                position + 1
            }
        }
    }
}

fn turn_dial_v1(mut position: i32, direction: Direction, num: i32) -> (i32, i32) {
    let mut num_0_clicks = 0;
    for _ in 0..num {
        position = turn_dial_once(position, direction);
        if position == 0 {
            num_0_clicks += 1
        }
    }
    (position, num_0_clicks)
}

fn turn_dial_v2(mut position: i32, direction: Direction, num: i32) -> (i32, i32) {
    let mut num_0_clicks = 0;

    match direction {
        Right => {
            num_0_clicks += (position + num) / 100;
            position = (position + num) % 100;
        }
        Left => {
            position = if (position - num) > 0 {
                position - num
            } else {
                if position - num == 0 {
                    num_0_clicks += 1;
                } else {
                    let extra_click = if position > 0 { 1 } else { 0 };
                    num_0_clicks += ((position - num) * -1) / 100 + extra_click;
                }

                (100 - (((position - num) * -1) % 100)) % 100
            }
        }
    }

    (position, num_0_clicks)
}

fn compare_turn_dial_fns() {
    let input = include_str!("input.txt");
    let mut position_1 = 50;
    let mut position_2 = 50;
    let mut num_0_clicks_1 = 0;
    let mut num_0_clicks_2 = 0;
    let mut break_now = false;

    for line in input.lines() {
        let num = line[1..].parse::<i32>().unwrap();
        let direction = if line.starts_with('R') { Right } else { Left };
        println!("position: {position_1}");
        println!("applying {direction:?}{num}");

        let (new_position1, new_num_0_clicks_1) = turn_dial_v1(position_1, direction, num);
        let (new_position2, new_num_0_clicks_2) = turn_dial_v2(position_2, direction, num);

        position_1 = new_position1;
        position_2 = new_position2;
        num_0_clicks_1 += new_num_0_clicks_1;
        num_0_clicks_2 += new_num_0_clicks_2;

        if position_1 != position_2 {
            println!("positions disagree: position1={position_1}, position2={position_2}");
            break_now = true;
        }
        if num_0_clicks_1 != num_0_clicks_2 {
            println!(
                "0 clocks disagree: num_0_click1={num_0_clicks_1}, num_0_click2={num_0_clicks_2}"
            );
            break_now = true;
        }
        if break_now {
            break;
        }
    }

    println!("positions agree: position1={position_1}, position2={position_2}");
    println!("0 clocks agree: num_0_click1={num_0_clicks_1}, num_0_click2={num_0_clicks_2}");
}
