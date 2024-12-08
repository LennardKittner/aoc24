#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use rayon::iter::Map;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelString;
use rayon::str::Lines;


fn check1(end_result: u64, current_result: u64, operants: &[u64]) -> bool {
    if end_result == current_result && operants.is_empty() {
        return true;
    }
    if end_result < current_result || operants.is_empty() {
        return false;
    }
    check1(end_result, current_result * operants[0], &operants[1..]) ||
        check1(end_result, current_result + operants[0], &operants[1..])
}

pub fn exec_day7_part1(input: &str) -> String {
    parse_input(input).map(|(result, operants)| {
        if check1(result, operants[0], &operants[1..]) {
            result
        } else {
            0
        }
    }).sum::<u64>().to_string()
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> Map<Lines<'_>, fn(&str) -> (u64, Vec<u64>)> {
    input.par_lines().map(|l| {
        let mut numbers = l.split_whitespace();
        let mut result = numbers.next().unwrap().to_string();
        result.pop();
        let result = u64::from_str(&result).unwrap();
        let rest = numbers.map(|n| u64::from_str(n).unwrap()).collect_vec();
        (result, rest)
    })
}

fn concat(o1: u64, o2: u64) -> u64 {
    (o1 * 10u64.pow(o2.ilog10()+1)) + o2
}

fn check2(end_result: u64, current_result: u64, operants: &[u64]) -> bool {
    if end_result == current_result && operants.is_empty() {
        return true;
    }
    if end_result < current_result || operants.is_empty() {
        return false;
    }
    check2(end_result, current_result + operants[0], &operants[1..]) ||
        check2(end_result, current_result * operants[0], &operants[1..]) ||
        check2(end_result, concat(current_result, operants[0]), &operants[1..])
}

pub fn exec_day7_part2(input: &str) -> String {
    parse_input(input).map(|(result, operants)| {
        if check2(result, operants[0], &operants[1..]) {
            result
        } else {
            0
        }
    }).sum::<u64>().to_string()
}

#[test]
fn test_day7_part1() {
    let input = match fs::read_to_string("./example/day7.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day7_part1(&input), "3749")
}

#[test]
fn test_day7_part2() {
    let input = match fs::read_to_string("./example/day7.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day7_part2(&input), "11387")
}

#[test]
fn test_concat() {
    assert_eq!(concat(123, 456), 123456);
    assert_eq!(concat(123, 1), 1231);
    assert_eq!(concat(123, 10), 12310);
    assert_eq!(concat(123, 100), 123100);
}
