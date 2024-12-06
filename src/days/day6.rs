#[cfg(test)]
use std::fs;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::iter::ParallelBridge;
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
    Visited(i64)
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
    fn get_direction(&self) -> Direction {
        match self {
            Guard(d) => *d,
            _ => panic!()
        }
    }
    fn is_guard(&self) -> bool {
        matches!(self, Guard(_))
    }
}

pub fn exec_day6_part1(input: &str) -> String {
    let (mut guard_position, mut grid) = parse_input(input);
    let num_visited = simulate_guard(&mut guard_position, &mut grid);
    num_visited.unwrap().to_string()
}

fn simulate_guard(guard_position: &mut (usize, usize), grid: &mut [Vec<State>]) -> Option<i64> {
    let mut num_visited = 1;
    loop {
        let direction = grid[guard_position.0][guard_position.1].direction_vector();
        let new_position = (guard_position.0 as i32 + direction.0, guard_position.1 as i32 + direction.1);
        if !(0..(grid.len() as i32)).contains(&new_position.0) || !(0..(grid[0].len() as i32)).contains(&new_position.1) {
            return Some(num_visited);
        }
        let current_state = grid[new_position.0 as usize][new_position.1 as usize];
        match current_state {
            Free => {
                let previous_direction = grid[guard_position.0][guard_position.1].get_direction();
                grid[guard_position.0][guard_position.1] = Visited(num_visited);
                *guard_position = (new_position.0 as usize, new_position.1 as usize);
                grid[guard_position.0][guard_position.1] = Guard(previous_direction);
                num_visited += 1;
            }
            Blocked => {
                let previous_state = grid[guard_position.0][guard_position.1];
                grid[guard_position.0][guard_position.1] =  previous_state.turn_right();
            }
            Visited(last_seen) => {
                let previous_direction = grid[guard_position.0][guard_position.1].get_direction();
                if last_seen == num_visited {
                    return None;
                }
                grid[guard_position.0][guard_position.1] = Visited(num_visited);
                *guard_position = (new_position.0 as usize, new_position.1 as usize);
                grid[guard_position.0][guard_position.1] = Guard(previous_direction);
            }
            _ => panic!()
        }
    }
}

pub fn exec_day6_part2(input: &str) -> String {
    let (initial_guard_position, mut comparison_grid) = parse_input(input);
    let const_grid = comparison_grid.clone();
    let mut guard_position = initial_guard_position;
    let _ = simulate_guard(&mut guard_position, &mut comparison_grid);
    comparison_grid[guard_position.0][guard_position.1] = Visited(0);
    comparison_grid[initial_guard_position.0][initial_guard_position.1] = Guard(Up);

    let num_loops: i32 = (0..comparison_grid.len()).cartesian_product(0..comparison_grid[0].len()).par_bridge().map(|(i, j)| {
        let mut guard_position = initial_guard_position;
        let mut grid = const_grid.clone();
        let start_state = comparison_grid[i][j];
        if start_state == Free || start_state == Blocked || start_state.is_guard() {
            return 0;
        }
        grid[i][j] = Blocked;
        if simulate_guard(&mut guard_position, &mut grid).is_some() {
            0
        } else {
            1
        }
    }).sum();

    num_loops.to_string()
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<State>>) {
    let mut guard_position = (0, 0);
    let grid = input.to_string().lines().map(|l| l.chars().map(|b| match b {
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
    (guard_position, grid)
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
