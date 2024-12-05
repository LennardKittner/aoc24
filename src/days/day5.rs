use std::collections::{HashMap, HashSet};
#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

fn find_passing_orders(input: &str) -> (i32, HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let input = input.split("\n\n").collect_vec();
    let rules = input[0].lines().map(|l| {
        let parts = l.split('|').collect_vec();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[1].parse().unwrap();
        (right, left)
    }).collect_vec();
    let mut dependencies: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        if dependencies.keys().contains(&rule.0) {
            dependencies.get_mut(&rule.0).unwrap().push(rule.1);
        } else {
            dependencies.insert(rule.0, vec![rule.1]);
        }
    }
    let prints = input[1].lines().map(|l| l.split(',').map(|n| i32::from_str(n).unwrap()).collect_vec()).collect_vec();
    let mut sum = 0;
    let mut not_passing = Vec::new();
    for print in prints {
        let mut seen = HashSet::new();
        let mut valid = true;
        for num in &print {
            seen.insert(num);
            if !dependencies.contains_key(num) {
                continue;
            }
            if !dependencies[num].iter().all(|n| seen.contains(n) || !print.contains(n)) {
                valid = false;
                break;
            }
        }
        if valid {
            sum += print[print.len()/2];
        } else {
            not_passing.push(print);
        }
    }
    (sum, dependencies, not_passing)
}

pub fn exec_day5_part1(input: &str) -> String {
    let (sum, _, _) = find_passing_orders(input);
    sum.to_string()
}

//TODO: this makes some shortcuts. Works for my input but not the example.
pub fn exec_day5_part2(input: &str) -> String {
    let (_, dependencies, mut wrong_prints) = find_passing_orders(input);

    for print in &mut wrong_prints {
        let mut i :i32 = 0;
        let mut seen = HashSet::new();

        while (i as usize) < print.len() {
            let num = print[i as usize];
            seen.insert(num);
            if !dependencies.contains_key(&num) {
                continue;
            }
            for n in dependencies[&num].iter() {
                if !seen.contains(n) && print.contains(n) {
                    let index = print.iter().position(|val| val == n).unwrap();
                    print[i as usize] = *n;
                    print[index] = num;
                    i = -1;
                    seen.clear();
                    break;
                }
            }
            i += 1;
        }
    }

    wrong_prints.iter().map(|p| p[p.len()/2]).sum::<i32>().to_string()
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
