use std::fs;

use crate::exec;

mod day1;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        1 => {
            exec(1, day1::exec_day1_part1, day1::exec_day1_part2, &input);
        },
        _ => (),
    }
}
