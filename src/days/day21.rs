use std::cmp::max;
#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use rayon::prelude::ParallelString;
use rayon::iter::ParallelIterator;

fn calc_step(secret: u64) -> u64 {
    let mix_and_prune = |old, new| {
        (old ^ new) % 16777216
    };
    let mut secret = secret;

    let mut tmp;
    tmp = secret * 64;
    secret = mix_and_prune(tmp, secret);
    tmp = secret / 32;
    secret = mix_and_prune(tmp, secret);
    tmp = secret * 2048;
    secret = mix_and_prune(tmp, secret);
    secret
}

fn calc_number(secret: u64, steps: u64) -> u64 {
    let mut current_number = secret;
    for _ in 0..steps {
        current_number = calc_step(current_number);
    }
    current_number
}

fn calc_changes_and_prices(secret: u64, steps: u64) -> (Vec<i8>, Vec<i8>) {
    let mut current_number = secret;
    let mut previous_price = (secret % 10) as i8;
    let mut changes = Vec::with_capacity(steps as usize-1);
    let mut prices = Vec::with_capacity(steps as usize-1);
    for _ in 0..steps {
        current_number = calc_step(current_number);
        let current_price = (current_number % 10) as i8;
        prices.push(current_price);
        changes.push(current_price - previous_price);
        previous_price = current_price;
    }
    (changes, prices)
}

pub fn exec_day21_part1(input: &str) -> String {
    input.trim().par_lines().map(|n| u64::from_str(n).unwrap()).map(|s| calc_number(s, 2000)).sum::<u64>().to_string()
}

fn generate_sequence(seed: u64) -> [[i8; 4]; 16] {
    let digit_3 = ((seed / 1000) % 10) as i8;
    let digit_2 = ((seed / 100) % 10) as i8;
    let digit_1 = ((seed / 10) % 10) as i8;
    let digit_0 = (seed % 10) as i8;
    [
        [-digit_3, -digit_2, -digit_1, digit_0],
        [-digit_3, -digit_2,  digit_1, digit_0],
        [-digit_3,  digit_2, -digit_1, digit_0],
        [-digit_3,  digit_2,  digit_1, digit_0],
        [ digit_3, -digit_2, -digit_1, digit_0],
        [ digit_3, -digit_2,  digit_1, digit_0],
        [ digit_3,  digit_2, -digit_1, digit_0],
        [ digit_3,  digit_2,  digit_1, digit_0],
        [-digit_3, -digit_2, -digit_1, -digit_0],
        [-digit_3, -digit_2,  digit_1, -digit_0],
        [-digit_3,  digit_2, -digit_1, -digit_0],
        [-digit_3,  digit_2,  digit_1, -digit_0],
        [ digit_3, -digit_2, -digit_1, -digit_0],
        [ digit_3, -digit_2,  digit_1, -digit_0],
        [ digit_3,  digit_2, -digit_1, -digit_0],
        [ digit_3,  digit_2,  digit_1, -digit_0],
    ]
}

pub fn exec_day21_part2(input: &str) -> String {
    let monkey_data = input.trim().lines().map(|n| u64::from_str(n).unwrap()).map(|s| calc_changes_and_prices(s, 2000)).collect_vec();
    let mut result = 0;
    for i in 0..=9999 {
        for target_sequence in generate_sequence(i) {
            let mut sum = 0;
            for (changes, prices) in &monkey_data {
                for i in 0..(changes.len() - 4) {
                    if changes[i..(i + 4)] == target_sequence {
                        sum += prices[i + 3];
                        break;
                    }
                }
            }
            result = max(result, sum);
        }
    }
    result.to_string()
}

#[test]
fn test_day21_part1() {
    let input = match fs::read_to_string("./example/day21.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day21_part1(&input), "37327623")
}

#[test]
fn test_day21_part2() {
    let input = match fs::read_to_string("./example/day21_2.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day21_part2(&input), "23")
}
