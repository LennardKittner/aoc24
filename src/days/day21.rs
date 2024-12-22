#[cfg(test)]
use std::fs;

pub fn exec_day21_part1(input: &str) -> String {
    "Not implemented".to_string()
}

pub fn exec_day21_part2(input: &str) -> String {
    "Not implemented".to_string()
}

#[test]
fn test_day21_part1() {
    let input = match fs::read_to_string("./example/day21.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day21_part1(&input), "TODO")
}

#[test]
fn test_day21_part2() {
    let input = match fs::read_to_string("./example/day21.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day21_part2(&input), "TODO")
}
