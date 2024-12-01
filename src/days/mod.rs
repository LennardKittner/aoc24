use std::{fs, i32};

use crate::exec;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
}
