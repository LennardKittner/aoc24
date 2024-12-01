use std::str::FromStr;
use itertools::Itertools;

pub fn exec_day1_part1(input: &str) -> String {
    let (l1, l2) = parse_input(input);
    let mut sum = 0;
    for i in 0..l1.len() {
        sum += (l1[i] - l2[i]).abs();
    }
    format!("{sum}")
}

pub fn exec_day1_part2(input: &str) -> String {
    let (l1, l2) = parse_input(input);
    let mut sum = 0;
    let mut occurrences = 0;
    let mut l1_index = 0;
    let mut l2_index = 0;
    while l2_index < l2.len() {
        if l1_index >= l1.len() {
            break
        }
        if l1[l1_index] < l2[l2_index] {
            sum += occurrences * l1[l1_index];
            l1_index += 1;
            while l1_index < l1.len() && l1[l1_index-1] == l1[l1_index] {
                sum += occurrences * l1[l1_index];
                l1_index += 1;
            }
            occurrences = 0;
            l2_index = l2_index.saturating_sub(1);
            continue
        }
        if l1[l1_index] == l2[l2_index] {
            occurrences += 1;
        }
        l2_index += 1;
    }
    if l1_index < l1.len() {
        sum += occurrences * l1[l1_index];
    }
    format!("{sum}")
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let (mut l1, mut l2): (Vec<i64>, Vec<i64>) = input.lines().map(|l| {
        let nums = l.split_whitespace().collect_vec();

        (i64::from_str(nums[0]).unwrap(), i64::from_str(nums[1]).unwrap())
    }).unzip();
    l1.sort();
    l2.sort();
    (l1, l2)
}
