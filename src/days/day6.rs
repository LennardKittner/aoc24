#[cfg(test)]
use std::fs;
use itertools::Itertools;
use crate::days::day6::Direction::{Up, Left, Right, Down};
use crate::days::day6::State::{Blocked, Free, Visited, Guard};

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
enum State {
    Guard(Direction),
    Free,
    Blocked,
    Visited(Direction)
}

impl State {
    fn direction_vector(&self) -> (i32, i32) {
        match self {
            Guard(Up) => (-1, 0),
            Guard(Left) => (0, -1),
            Guard(Right) => (0, 1),
            Guard(Down) => (1, 0),
            s => panic!("Invalid state {s:?}")
        }
    }
    fn turn_right(&self) -> State {
        match self {
            Guard(Up) => Guard(Right),
            Guard(Left) => Guard(Up),
            Guard(Right) => Guard(Down),
            Guard(Down) => Guard(Left),
            _ => panic!()
        }
    }
    fn is_visited(&self) -> bool {
        match self {
            Visited(_) => true,
            _ => false
        }
    }
    fn get_direction(&self) -> Direction {
        match self {
            Guard(d) => *d,
            Visited(d) => *d,
            _ => panic!()
        }
    }
    fn is_guard(&self) -> bool {
        match self {
            Guard(_) => true,
            _ => false
        }
    }
}

pub fn exec_day6_part1(input: &str) -> String {
    let mut guard_position = (0, 0);
    let mut grid = input.to_string().lines().map(|l| l.chars().map(|b| match b {
        '.' => Free,
        '^' => Guard(Up),
        '#' => Blocked,
        _ => panic!("invalid char")
    }).collect_vec()
    ).collect_vec();
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Guard(Up) {
                guard_position = (i, j);
                break 'outer;
            }
        }
    }
    let mut num_visited = 1;
    loop {
        let direction = grid[guard_position.0][guard_position.1].direction_vector();
        let new_position = (guard_position.0 as i32 + direction.0, guard_position.1 as i32 + direction.1);
        if !(0..(grid.len() as i32)).contains(&new_position.0) || !(0..(grid[0].len() as i32)).contains(&new_position.1) {
            break;
        }
        let current_state = grid[new_position.0 as usize][new_position.1 as usize];
        match current_state {
            Free => {
                let previous_direction = grid[guard_position.0][guard_position.1].get_direction();
                grid[guard_position.0][guard_position.1] = Visited(previous_direction);
                guard_position = (new_position.0 as usize, new_position.1 as usize);
                grid[guard_position.0][guard_position.1] = Guard(previous_direction);
                num_visited += 1;
            }
            Blocked => {
                let previous_direction = grid[guard_position.0][guard_position.1];
                grid[guard_position.0][guard_position.1] = previous_direction.turn_right();
            }
            Visited(_) => {
                let previous_direction = grid[guard_position.0][guard_position.1].get_direction();
                grid[guard_position.0][guard_position.1] = Visited(previous_direction);
                guard_position = (new_position.0 as usize, new_position.1 as usize);
                grid[guard_position.0][guard_position.1] = Guard(previous_direction);
            }
            _ => panic!()
        }
    }
    num_visited.to_string()
}

pub fn exec_day6_part2(input: &str) -> String {
    let mut initial_guard_position = (0, 0);
    let mut initial_grid = input.to_string().lines().map(|l| l.chars().map(|b| match b {
        '.' => Free,
        '^' => Guard(Up),
        '#' => Blocked,
        _ => panic!("invalid char")
    }).collect_vec()
    ).collect_vec();
    'outer: for i in 0..initial_grid.len() {
        for j in 0..initial_grid[0].len() {
            if initial_grid[i][j] == Guard(Up) {
                initial_guard_position = (i, j);
                break 'outer;
            }
        }
    }
    let const_grid = initial_grid.clone();
    let mut guard_position = initial_guard_position;
    loop {
        let direction = initial_grid[guard_position.0][guard_position.1].direction_vector();
        let new_position = (guard_position.0 as i32 + direction.0, guard_position.1 as i32 + direction.1);
        if !(0..(initial_grid.len() as i32)).contains(&new_position.0) || !(0..(initial_grid[0].len() as i32)).contains(&new_position.1) {
            break;
        }
        let current_state = initial_grid[new_position.0 as usize][new_position.1 as usize];
        match current_state {
            Free => {
                let previous_direction = initial_grid[guard_position.0][guard_position.1].get_direction();
                initial_grid[guard_position.0][guard_position.1] = Visited(previous_direction);
                guard_position = (new_position.0 as usize, new_position.1 as usize);
                initial_grid[guard_position.0][guard_position.1] = Guard(previous_direction);
            }
            Blocked => {
                let previous_direction = initial_grid[guard_position.0][guard_position.1];
                initial_grid[guard_position.0][guard_position.1] = previous_direction.turn_right();
            }
            Visited(_) => {
                let previous_direction = initial_grid[guard_position.0][guard_position.1].get_direction();
                initial_grid[guard_position.0][guard_position.1] = Visited(previous_direction);
                guard_position = (new_position.0 as usize, new_position.1 as usize);
                initial_grid[guard_position.0][guard_position.1] = Guard(previous_direction);
            }
            _ => panic!()
        }
    }
    initial_grid[guard_position.0][guard_position.1] = Visited(Down);
    initial_grid[initial_guard_position.0][initial_guard_position.1] = Guard(Up);

    let mut num_loops = 0;
    let mut tries = -1;
    let mut num_loops_early_out = 0;
    for i in 0..initial_grid.len() {
        for j in 0..initial_grid[0].len() {
            let mut guard_position = initial_guard_position;
            let mut grid = const_grid.clone();
            let previous_state = initial_grid[i][j];
            if previous_state == Free || previous_state == Blocked || previous_state.is_guard() {
                continue;
            }
            tries += 1;
            grid[i][j] = Blocked;
            let mut found_loop = true;
            for _ in 0..(grid.len()*50) {
                let direction = grid[guard_position.0][guard_position.1].direction_vector();
                let new_position = (guard_position.0 as i32 + direction.0, guard_position.1 as i32 + direction.1);
                if !(0..(grid.len() as i32)).contains(&new_position.0) || !(0..(grid[0].len() as i32)).contains(&new_position.1) {
                    found_loop = false;
                    break;
                }
                let current_state = grid[new_position.0 as usize][new_position.1 as usize];
                match current_state {
                    Free => {
                        let previous_direction = grid[guard_position.0][guard_position.1].get_direction();
                        grid[guard_position.0][guard_position.1] = Visited(previous_direction);
                        guard_position = (new_position.0 as usize, new_position.1 as usize);
                        grid[guard_position.0][guard_position.1] = Guard(previous_direction);
                    }
                    Blocked => {
                        let previous_state = grid[guard_position.0][guard_position.1];
                        grid[guard_position.0][guard_position.1] =  previous_state.turn_right();
                    }
                    Visited(old_direction) => {
                        let previous_direction = grid[guard_position.0][guard_position.1].get_direction();
                        if old_direction == previous_direction {
                            num_loops_early_out += 1;
                            break;
                        }
                        grid[guard_position.0][guard_position.1] = Visited(previous_direction);
                        guard_position = (new_position.0 as usize, new_position.1 as usize);
                        grid[guard_position.0][guard_position.1] = Guard(previous_direction);
                    }
                    _ => panic!()
                }
            }
            if found_loop {
                num_loops += 1;
            }
        }
    }
    println!("{num_loops_early_out}");
    num_loops.to_string()
}

#[test]
fn test_day6_part1() {
    let input = match fs::read_to_string("./example/day6.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day6_part1(&input), "41")
}

#[test]
fn test_day6_part2() {
    let input = match fs::read_to_string("./example/day6.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day6_part2(&input), "6")
}
