use std::str::FromStr;
use itertools::Itertools;

pub fn exec_day2_part1(input: &str) -> String {
    let reports = input.lines().map(|line| {
        line.split_whitespace().map(|level| i64::from_str(level).unwrap()).collect_vec()
    }).collect_vec();
    let mut sum = 0;
    for report in reports {
        if check_report(&report) {
            sum += 1;
        }
    }
    sum.to_string()
}

fn check_report(report: &[i64]) -> bool {
    if report[0] == report[1] {
        return false;
    }
    if report[0] < report[1] {
        for level_idx in 1..report.len() {
            if report[level_idx - 1] >= report[level_idx] || report[level_idx - 1].abs_diff(report[level_idx]) > 3 {
                return false;
            }
        }
    } else {
        for level_idx in 1..report.len() {
            if report[level_idx - 1] <= report[level_idx] || report[level_idx - 1].abs_diff(report[level_idx]) > 3 {
                return false;
            }
        }
    };
    true
}

pub fn exec_day2_part2(input: &str) -> String {
    let reports = input.lines().map(|line| {
        line.split_whitespace().map(|level| i64::from_str(level).unwrap()).collect_vec()
    }).collect_vec();
    let mut sum = 0;
    for report in reports {
        for level_to_remove in 0..report.len() {
            let mut report = report.clone();
            report.remove(level_to_remove);
            if check_report(&report) {
                sum += 1;
                break;
            }
        }
    }
    sum.to_string()
}
