#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge};
use crate::days::day7::Operator::{Addition, Concat, Multiply};
use rayon::iter::ParallelIterator;

#[derive(Copy, Clone)]
enum Operator {
    Multiply,
    Addition,
    Concat
}

pub fn exec_day7_part1(input: &str) -> String {
    let equations = input.lines().map(|l| {
        let numbers = l.split_whitespace().collect_vec();
        let mut result = numbers[0].to_string();
        result.pop();
        let result = u64::from_str(&result).unwrap();
        let rest = numbers.iter().rev().take(numbers.len()-1).map(|n| u64::from_str(n).unwrap()).rev().collect_vec();
        (result, rest)
    }).collect_vec();
    let max_len = equations.iter().map(|(_, rest)| rest.len()).max().unwrap();
    let mut operators = Vec::new();
    for _ in 0..max_len {
        operators.push(Multiply);
        operators.push(Addition);
    }

    equations.par_iter().map(|(result, operants)| {
        if (0..2u64.pow(operants.len() as u32)).par_bridge().any(|ops| {
            evaluate(&generate_operator_vec(ops, operants.len()), operants.as_slice()) == *result
        }) {
            *result
        } else {
            0
        }
    }).sum::<u64>().to_string()
}

fn evaluate(operators: &[Operator], operants: &[u64]) -> u64 {
    let mut result = operants[0];
    let mut iter = operants.iter();
    iter.next(); // skip first
    for (operant, operator) in iter.zip(operators) {
        match operator {
            Multiply => result *= operant,
            Addition => result += operant,
            Concat => result = u64::from_str(&format!("{}{}", result, operant)).unwrap()
        }
    }
    result
}

fn generate_operator_vec(val: u64, len: usize) -> Vec<Operator> {
    let mut result = Vec::new();
    let mut val = val;
    for _ in 0..len {
        if val & 1 == 1 {
            result.push(Multiply)
        } else {
            result.push(Addition)
        }
        val = val >> 1;
    }
    result
}

fn generate_operator_vec3(val: u64, len: usize) -> Vec<Operator> {
    let mut result = Vec::new();
    let mut val = val;
    for _ in 0..len {
        if val & 0b11 == 1 {
            result.push(Multiply)
        } else if val & 0b11 == 2 {
            result.push(Addition)
        } else if val & 0b11 == 3 {
            result.push(Concat)
        } else {
            result.push(Addition)
        }
        val = val >> 2;
    }
    result
}

pub fn exec_day7_part2(input: &str) -> String {
    let equations = input.lines().map(|l| {
        let numbers = l.split_whitespace().collect_vec();
        let mut result = numbers[0].to_string();
        result.pop();
        let result = u64::from_str(&result).unwrap();
        let rest = numbers.iter().rev().take(numbers.len()-1).map(|n| u64::from_str(n).unwrap()).rev().collect_vec();
        (result, rest)
    }).collect_vec();
    let max_len = equations.iter().map(|(_, rest)| rest.len()).max().unwrap();
    let mut operators = Vec::new();
    for _ in 0..max_len {
        operators.push(Multiply);
        operators.push(Addition);
    }

    equations.par_iter().map(|(result, operants)| {
        if (0..2u64.pow(2 * operants.len() as u32)).par_bridge().any(|ops| {
            evaluate(&generate_operator_vec3(ops, operants.len()), operants.as_slice()) == *result
        }) {
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
