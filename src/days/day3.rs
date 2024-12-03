use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;

pub fn exec_day3_part1(input: &str) -> String {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for capture in regex.captures_iter(&input) {
        let result: (&str, [&str; 2]) = capture.extract();
        sum += i64::from_str(result.1[0]).unwrap() * i64::from_str(result.1[1]).unwrap();
    }
    sum.to_string()
}

pub fn exec_day3_part2(input: &str) -> String {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for capture in regex.captures_iter(&input) {
        let mut arg1 = "";
        let mut arg2 = "";
        let result = capture.iter().filter_map(|m| m.map(|m| m.as_str())).collect_vec();
        let name = result[0];
        if result.len() > 2 {
            arg1 = result[1];
            arg2 = result[2];
        }
        if name.starts_with("don't") {
            enabled = false;
        } else if name.starts_with("do") {
            enabled = true;
        } else if enabled {
            sum += i64::from_str(arg1).unwrap() * i64::from_str(arg2).unwrap();
        }
    }
    sum.to_string()
}
