use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

fn even_digit(num: u64) -> bool {
    num > 9 && num.ilog10() % 2 == 1
}

fn split_digits(num: u64) -> (u64, u64) {
    let divider = 10u64.pow((num.ilog10()+1) / 2);
    (num / divider, num % divider)
}

fn insert_stone(new_stones: &mut HashMap<u64, u64>, stone: (u64, u64)) {
    if let Vacant(e) = new_stones.entry(stone.0) {
        e.insert(stone.1);
    } else {
        *new_stones.get_mut(&stone.0).unwrap() += stone.1;
    }
}

pub fn exec_day11_part1(input: &str) -> String {
    let input = input.split_whitespace().map(|n| u64::from_str(n).unwrap()).collect_vec();
   simulate_stones(&input, 25).to_string()
}

pub fn exec_day11_part2(input: &str) -> String {
    let input = input.split_whitespace().map(|n| u64::from_str(n).unwrap()).collect_vec();
    simulate_stones(&input, 75).to_string()
}

fn simulate_stones(input: &[u64], rounds: u64) -> u64 {
    let mut stones = HashMap::new();
    for &stone in input {
        stones.insert(stone, 1);
    }
    for _ in 0..rounds {
        let mut new_stones = HashMap::new();
        for stone in &stones {
            if *stone.0 == 0 {
                insert_stone(&mut new_stones, (1, *stone.1));
            } else if even_digit(*stone.0) {
                let (s1, s2) = split_digits(*stone.0);
                insert_stone(&mut new_stones, (s1, *stone.1));
                insert_stone(&mut new_stones, (s2, *stone.1));
            } else {
                insert_stone(&mut new_stones, (stone.0 * 2024, *stone.1));
            }
        }
        stones = new_stones;
    }
    stones.values().sum::<u64>()
}

#[test]
fn test_day11_part1() {
    let input = match fs::read_to_string("./example/day11.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day11_part1(&input), "55312")
}

#[test]
fn test_day11_part2() {
    let input = match fs::read_to_string("./example/day11.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day11_part2(&input), "65601038650482")
}
