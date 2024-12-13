#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

// a -> ax
// c -> ay
// b -> bx
// d -> by
// g -> px
// f -> py

fn solve_equations((a, c): (i64, i64), (b, d): (i64, i64), (g, f): (i64, i64)) -> Option<(i64, i64)> {
    let solution;
    if a != 0 && b*c - d*g != 0 {
        let det = b*c - a*d;
        solution = ((b*f - d*g) / det, (c*g - a*f) / det)
    } else {
        panic!("I only implemented one of the possible solutions as this sufficed for my input and I assumed it would work for any input")
    }
    if solution.0*a + solution.1*b == g && solution.0*c + solution.1*d == f {
        Some(solution)
    } else {
        None
    }
}

pub fn exec_day13_part1(input: &str) -> String {
    let machines = input.split("\n\n").map(|machine| {
        let mut lines = machine.lines();
        let mut button_a = lines.next().unwrap()[12..].split(", Y+");
        let xa = button_a.next().unwrap();
        let ya = button_a.next().unwrap();
        let mut button_a = lines.next().unwrap()[12..].split(", Y+");
        let xb = button_a.next().unwrap();
        let yb = button_a.next().unwrap();
        let mut price = lines.next().unwrap()[9..].split(", Y=");
        let xp = price.next().unwrap();
        let yp = price.next().unwrap();
        ((i64::from_str(xa).unwrap(), i64::from_str(ya).unwrap()), (i64::from_str(xb).unwrap(), i64::from_str(yb).unwrap()), (i64::from_str(xp).unwrap(), i64::from_str(yp).unwrap()))
    }).collect_vec();
    let mut result = 0;
    for machine in machines {
        if let Some((a, b)) = solve_equations(machine.0, machine.1, machine.2) {
            result += a*3 + b;
        }
    }
    result.to_string()
}

pub fn exec_day13_part2(input: &str) -> String {
    let machines = input.split("\n\n").map(|machine| {
        let mut lines = machine.lines();
        let mut button_a = lines.next().unwrap()[12..].split(", Y+");
        let xa = button_a.next().unwrap();
        let ya = button_a.next().unwrap();
        let mut button_a = lines.next().unwrap()[12..].split(", Y+");
        let xb = button_a.next().unwrap();
        let yb = button_a.next().unwrap();
        let mut price = lines.next().unwrap()[9..].split(", Y=");
        let xp = price.next().unwrap();
        let yp = price.next().unwrap();
        ((i64::from_str(xa).unwrap(), i64::from_str(ya).unwrap()), (i64::from_str(xb).unwrap(), i64::from_str(yb).unwrap()), (i64::from_str(xp).unwrap()+10000000000000, i64::from_str(yp).unwrap()+10000000000000))
    }).collect_vec();
    let mut result = 0;
    for machine in machines {
        if let Some((a, b)) = solve_equations(machine.0, machine.1, machine.2) {
            result += a*3 + b;
        }
    }
    result.to_string()
}

#[test]
fn test_day13_part1() {
    let input = match fs::read_to_string("./example/day13.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day13_part1(&input), "480")
}

#[test]
fn test_day13_part2() {
    let input = match fs::read_to_string("./example/day13.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day13_part2(&input), "875318608908")
}
