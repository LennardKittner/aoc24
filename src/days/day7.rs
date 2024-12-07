#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use crate::days::day7::Operator::{Addition, Concat, Multiply};
use rayon::iter::ParallelIterator;

#[derive(Copy, Clone)]
enum Operator {
    Multiply,
    Addition,
    Concat
}

fn check1(operator: Operator, end_result: u64, current_result: u64, operants: &[u64]) -> bool {
    if end_result == current_result && operants.is_empty() {
        return true;
    }
    if end_result < current_result || operants.is_empty() {
        return false;
    }
    match operator {
        Multiply => {
            check1(Multiply, end_result, current_result * operants[0], &operants[1..]) ||
                check1(Addition, end_result, current_result * operants[0], &operants[1..])
        }
        Addition => {
            check1(Multiply, end_result, current_result + operants[0], &operants[1..]) ||
                check1(Addition, end_result, current_result + operants[0], &operants[1..])
        }
        _ => panic!()
    }
}

pub fn exec_day7_part1(input: &str) -> String {
    let equations = parse_input(input);

    equations.par_iter().map(|(result, operants)| {
        if check1(Multiply, *result, operants[0], &operants[1..]) ||
            check1(Addition, *result, operants[0], &operants[1..]) {
            *result
        } else {
            0
        }
    }).sum::<u64>().to_string()
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(|l| {
        let numbers = l.split_whitespace().collect_vec();
        let mut result = numbers[0].to_string();
        result.pop();
        let result = u64::from_str(&result).unwrap();
        let rest = numbers.iter().rev().take(numbers.len() - 1).map(|n| u64::from_str(n).unwrap()).rev().collect_vec();
        (result, rest)
    }).collect_vec()
}

fn concat(o1: u64, o2: u64) -> u64 {
    (o1 * 10u64.pow(o2.ilog10()+1)) + o2
}

fn check2(operator: Operator, end_result: u64, current_result: u64, operants: &[u64]) -> bool {
    if end_result == current_result && operants.is_empty() {
        return true;
    }
    if end_result < current_result || operants.is_empty() {
        return false;
    }
    match operator {
        Multiply => {
            check2(Multiply, end_result, current_result * operants[0], &operants[1..]) ||
                check2(Addition, end_result, current_result * operants[0], &operants[1..]) ||
                check2(Concat, end_result, current_result * operants[0], &operants[1..])
        }
        Addition => {
            check2(Multiply, end_result, current_result + operants[0], &operants[1..]) ||
                check2(Addition, end_result, current_result + operants[0], &operants[1..]) ||
                check2(Concat, end_result, current_result + operants[0], &operants[1..])
        }
        Concat => {
            check2(Multiply, end_result, concat(current_result, operants[0]), &operants[1..]) ||
                check2(Addition, end_result, concat(current_result, operants[0]), &operants[1..]) ||
                check2(Concat, end_result, concat(current_result, operants[0]), &operants[1..])
        }
    }
}

pub fn exec_day7_part2(input: &str) -> String {
    let equations = parse_input(input);

    equations.par_iter().map(|(result, operants)| {
        if check2(Multiply, *result, operants[0], &operants[1..]) ||
            check2(Addition, *result, operants[0], &operants[1..]) ||
            check2(Concat, *result, operants[0], &operants[1..]){
            *result
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
