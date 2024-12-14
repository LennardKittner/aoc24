#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

pub fn exec_day14_part1(input: &str) -> String {
    let (robots, mut robot_current_positions) = parse_input(input);
    #[cfg(test)] const DIMENSION_X: i64  = 11;
    #[cfg(test)] const DIMENSION_Y: i64 = 7;
    #[cfg(not(test))] const DIMENSION_X: i64 = 101;
    #[cfg(not(test))] const DIMENSION_Y: i64 = 103;

    let simulation_steps = 100;

    for (robot_id, (_, (vx, vy))) in robots.iter().enumerate() {
        let mut current_position = robot_current_positions[robot_id];
        current_position = (((current_position.0.wrapping_add(*vx * simulation_steps) % DIMENSION_X) + DIMENSION_X) % DIMENSION_X, ((current_position.1.wrapping_add(*vy * simulation_steps) % DIMENSION_Y) + DIMENSION_Y) % DIMENSION_Y);
        robot_current_positions[robot_id] = current_position;
    }

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    for (x, y) in robot_current_positions {
        if x < DIMENSION_X / 2 && y < DIMENSION_Y / 2 {
            top_left += 1;
        } else if x > DIMENSION_X / 2 && y > DIMENSION_Y / 2 {
            bottom_right += 1;
        } else if x < DIMENSION_X / 2 && y > DIMENSION_Y / 2 {
            bottom_left += 1;
        } else if x > DIMENSION_X / 2 && y < DIMENSION_Y / 2 {
            top_right += 1;
        } else {
            // we don't care about the middle
        }
    }

    (top_left*top_right*bottom_left*bottom_right).to_string()
}

pub fn exec_day14_part2(input: &str) -> String {
    let (robots, mut robot_current_positions) = parse_input(input);
    const DIMENSION_X: i64 = 101;
    const DIMENSION_Y: i64 = 103;

    let mut simulation_step = 0;
    const ZERO_LINES: [[bool; DIMENSION_X as usize]; DIMENSION_Y as usize] = [[false; DIMENSION_X as usize]; DIMENSION_Y as usize];
    let mut lines;

    'outer: loop {
        lines = ZERO_LINES;
        for (robot_id, (_, (vx, vy))) in robots.iter().enumerate() {
            let mut current_position = robot_current_positions[robot_id];
            lines[current_position.1 as usize][current_position.0 as usize] = true;
            current_position = (((current_position.0.wrapping_add(*vx) % DIMENSION_X) + DIMENSION_X) % DIMENSION_X, ((current_position.1.wrapping_add(*vy) % DIMENSION_Y) + DIMENSION_Y) % DIMENSION_Y);
            robot_current_positions[robot_id] = current_position;
        }

        for line in lines {
            let mut true_in_a_row = 0;
            for b in line {
                if b && true_in_a_row > 0 {
                    true_in_a_row += 1;
                } else if b {
                    true_in_a_row = 1;
                } else if !b {
                    if true_in_a_row >= 31 {
                        break 'outer;
                    }
                    true_in_a_row = 0;
                }
            }
        }
        simulation_step += 1;
    }

    simulation_step.to_string()
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<((i64, i64), (i64, i64))>, Vec<(i64, i64)>) {
    let robots = input.lines().map(|l| {
        let mut tmp = l[2..].split_whitespace();
        let position = tmp.next().unwrap();
        let velocity = tmp.next().unwrap();
        let mut position = position.split(',');
        let mut velocity = velocity[2..].split(',');
        ((i64::from_str(position.next().unwrap()).unwrap(), i64::from_str(position.next().unwrap()).unwrap()), (i64::from_str(velocity.next().unwrap()).unwrap(), i64::from_str(velocity.next().unwrap()).unwrap()))
    }).collect_vec();
    let mut robot_current_positions = Vec::new();
    for ((x, y), _) in &robots {
        robot_current_positions.push((*x, *y));
    }
    (robots, robot_current_positions)
}

#[test]
fn test_day14_part1() {
    let input = match fs::read_to_string("./example/day14.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day14_part1(&input), "12")
}

#[test]
fn test_day14_part2() {
    let input = match fs::read_to_string("./input/day14.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day14_part2(&input), "6516")
}
