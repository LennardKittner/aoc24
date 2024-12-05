use std::fs;

use crate::exec;

mod day3;
mod day1;
mod day5;
mod day4;
mod day2;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        3 => {
            exec(3, day3::exec_day3_part1, day3::exec_day3_part2, &input);
        },
        1 => {
            exec(1, day1::exec_day1_part1, day1::exec_day1_part2, &input);
        },
        5 => {
            exec(5, day5::exec_day5_part1, day5::exec_day5_part2, &input);
        },
        4 => {
            exec(4, day4::exec_day4_part1, day4::exec_day4_part2, &input);
        },
        2 => {
            exec(2, day2::exec_day2_part1, day2::exec_day2_part2, &input);
        },
        _ => (),
    }
}
