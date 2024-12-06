use std::cmp::Ordering;
use std::collections::HashMap;
#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

fn find_passing_orders(input: &str) -> (i32, i32) {
    let input = input.split("\n\n").collect_vec();
    let rules = input[0].lines().map(|l| {
        let parts = l.split('|').collect_vec();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[1].parse().unwrap();
        (right, left)
    }).collect_vec();
    let mut rule_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        if rule_map.keys().contains(&rule.0) {
            rule_map.get_mut(&rule.0).unwrap().push(rule.1);
        } else {
            rule_map.insert(rule.0, vec![rule.1]);
        }
    }
    let prints = input[1].lines().map(|l| l.split(',').map(|n| i32::from_str(n).unwrap()).collect_vec()).collect_vec();
    let mut sum_correct = 0;
    let mut sum_wrong = 0;
    for print in prints {
        let mut print_c = print.clone();
        print_c.sort_unstable_by(|o1, o2| {
            if let Some(set) = rule_map.get(o1) {
                if set.contains(o2) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if let Some(set) = rule_map.get(o2) {
                if set.contains(o1) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        });
        if print_c == print {
            sum_correct += print[print.len()/2];
        } else {
            sum_wrong += print_c[print.len()/2]
        }
    }
    (sum_correct, sum_wrong)
}

pub fn exec_day5_part1(input: &str) -> String {
    let (sum, _,) = find_passing_orders(input);
    sum.to_string()
}

pub fn exec_day5_part2(input: &str) -> String {
    let (_, sum) = find_passing_orders(input);
    sum.to_string()
}

#[test]
fn test_day5_part1() {
    let input = match fs::read_to_string("./example/day5.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day5_part1(&input), "143")
}

#[test]
fn test_day5_part2() {
    let input = match fs::read_to_string("./example/day5.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day5_part2(&input), "123")
}
