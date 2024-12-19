use std::collections::HashMap;
#[cfg(test)]
use std::fs;
use itertools::Itertools;

fn is_possible<'a>(towels: &[&str], target: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
    if cache.contains_key(&target) {
        return cache[&target];
    }
    if target.is_empty() {
        return true;
    }
    if towels.iter().any(|&t| {
        if let Some(suffix) = target.strip_prefix(t) {
            is_possible(towels, suffix, cache)
        } else {
            false
        }
    }) {
        cache.insert(target, true);
        true
    } else {
        cache.insert(target, false);
        false
    }
}

fn is_possible2<'a>(towels: &[&str], target: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if cache.contains_key(&target) {
        return cache[&target];
    }
    if target.is_empty() {
        return 1;
    }
    let result = towels.iter().map(|&t| {
        if let Some(suffix) = target.strip_prefix(t) {
            is_possible2(towels, suffix, cache)
        } else {
            0
        }
    }).sum();
    cache.insert(target, result);
    result
}

pub fn exec_day19_part1(input: &str) -> String {
    let (towels, target) = input.trim().split("\n\n").collect_tuple().unwrap();
    let towels = towels.split(", ").collect_vec();
    let targets = target.lines().collect_vec();
    let mut cache = HashMap::new();

    let mut result = 0;
    for target in targets {
        if is_possible(&towels, target, &mut cache) {
            result += 1;
        }
    }

    result.to_string()
}

pub fn exec_day19_part2(input: &str) -> String {
    let (towels, target) = input.trim().split("\n\n").collect_tuple().unwrap();
    let towels = towels.split(", ").collect_vec();
    let targets = target.lines().collect_vec();
    let mut cache = HashMap::new();

    let mut result = 0;
    for target in targets {
        result += is_possible2(&towels, target, &mut cache);
    }

    result.to_string()
}

#[test]
fn test_day19_part1() {
    let input = match fs::read_to_string("./example/day19.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day19_part1(&input), "6")
}

#[test]
fn test_day19_part2() {
    let input = match fs::read_to_string("./example/day19.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day19_part2(&input), "16")
}
