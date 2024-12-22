use std::fs;

use crate::exec;

mod day20;
mod day14;
mod day9;
mod day10;
mod day11;
mod day21;
mod day15;
mod day8;
mod day3;
mod day7;
mod day6;
mod day2;
mod day18;
mod day5;
mod day1;
mod day19;
mod day4;
mod day12;
mod day16;
mod day17;
mod day13;
mod day22;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        20 => {
            exec(20, day20::exec_day20_part1, day20::exec_day20_part2, &input);
        },
        14 => {
            exec(14, day14::exec_day14_part1, day14::exec_day14_part2, &input);
        },
        9 => {
            exec(9, day9::exec_day9_part1, day9::exec_day9_part2, &input);
        },
        10 => {
            exec(10, day10::exec_day10_part1, day10::exec_day10_part2, &input);
        },
        11 => {
            exec(11, day11::exec_day11_part1, day11::exec_day11_part2, &input);
        },
        21 => {
            exec(21, day21::exec_day21_part1, day21::exec_day21_part2, &input);
        },
        15 => {
            exec(15, day15::exec_day15_part1, day15::exec_day15_part2, &input);
        },
        8 => {
            exec(8, day8::exec_day8_part1, day8::exec_day8_part2, &input);
        },
        3 => {
            exec(3, day3::exec_day3_part1, day3::exec_day3_part2, &input);
        },
        7 => {
            exec(7, day7::exec_day7_part1, day7::exec_day7_part2, &input);
        },
        6 => {
            exec(6, day6::exec_day6_part1, day6::exec_day6_part2, &input);
        },
        2 => {
            exec(2, day2::exec_day2_part1, day2::exec_day2_part2, &input);
        },
        18 => {
            exec(18, day18::exec_day18_part1, day18::exec_day18_part2, &input);
        },
        5 => {
            exec(5, day5::exec_day5_part1, day5::exec_day5_part2, &input);
        },
        1 => {
            exec(1, day1::exec_day1_part1, day1::exec_day1_part2, &input);
        },
        19 => {
            exec(19, day19::exec_day19_part1, day19::exec_day19_part2, &input);
        },
        4 => {
            exec(4, day4::exec_day4_part1, day4::exec_day4_part2, &input);
        },
        12 => {
            exec(12, day12::exec_day12_part1, day12::exec_day12_part2, &input);
        },
        16 => {
            exec(16, day16::exec_day16_part1, day16::exec_day16_part2, &input);
        },
        17 => {
            exec(17, day17::exec_day17_part1, day17::exec_day17_part2, &input);
        },
        13 => {
            exec(13, day13::exec_day13_part1, day13::exec_day13_part2, &input);
        },
        22 => {
            exec(22, day22::exec_day22_part1, day22::exec_day22_part2, &input);
        },
        _ => (),
    }
}
