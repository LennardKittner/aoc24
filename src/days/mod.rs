use std::fs;

use crate::exec;

mod day9;
mod day3;
mod day6;
mod day1;
mod day8;
mod day5;
mod day10;
mod day7;
mod day4;
mod day2;
mod day11;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        9 => {
            exec(9, day9::exec_day9_part1, day9::exec_day9_part2, &input);
        },
        3 => {
            exec(3, day3::exec_day3_part1, day3::exec_day3_part2, &input);
        },
        6 => {
            exec(6, day6::exec_day6_part1, day6::exec_day6_part2, &input);
        },
        1 => {
            exec(1, day1::exec_day1_part1, day1::exec_day1_part2, &input);
        },
        8 => {
            exec(8, day8::exec_day8_part1, day8::exec_day8_part2, &input);
        },
        5 => {
            exec(5, day5::exec_day5_part1, day5::exec_day5_part2, &input);
        },
        10 => {
            exec(10, day10::exec_day10_part1, day10::exec_day10_part2, &input);
        },
        7 => {
            exec(7, day7::exec_day7_part1, day7::exec_day7_part2, &input);
        },
        4 => {
            exec(4, day4::exec_day4_part1, day4::exec_day4_part2, &input);
        },
        2 => {
            exec(2, day2::exec_day2_part1, day2::exec_day2_part2, &input);
        },
        11 => {
            exec(11, day11::exec_day11_part1, day11::exec_day11_part2, &input);
        },
        _ => (),
    }
}
