#[cfg(test)]
use std::fs;
use itertools::Itertools;
use crate::days::day15::Entity::{Free, Wall, Boxx, Robot};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Entity {
    Wall,
    Boxx,
    Robot,
    Free
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Entity2 {
    Wall,
    BoxL,
    BoxR,
    Robot,
    Free
}

fn simulate_box_move(grid: &mut [Vec<Entity>], current_position: (i32, i32), direction: (i32, i32)) -> (i32, i32) {
    let mut current_entity = Boxx;
    let mut tmp_current_position = (current_position.0 + direction.0, current_position.1 + direction.1);
    while current_entity == Boxx {
        tmp_current_position = (tmp_current_position.0 + direction.0, tmp_current_position.1 + direction.1);
        current_entity = grid[tmp_current_position.1 as usize][tmp_current_position.0 as usize];
    }
    if current_entity == Wall {
        return current_position;
    }
    while current_entity != Robot {
        grid[tmp_current_position.1  as usize][tmp_current_position.0  as usize] = Boxx;
        tmp_current_position = (tmp_current_position.0 - direction.0, tmp_current_position.1 - direction.1);
        current_entity = grid[tmp_current_position.1  as usize][tmp_current_position.0  as usize];
    }
    grid[tmp_current_position.1  as usize][tmp_current_position.0  as usize] = Free;
    tmp_current_position = (current_position.0 + direction.0, current_position.1 + direction.1);
    grid[tmp_current_position.1  as usize][tmp_current_position.0  as usize] = Robot;

    tmp_current_position
}

pub fn exec_day15_part1(input: &str) -> String {
    let input = input.trim();
    let (grid, moves) = input.split("\n\n").collect_tuple().unwrap();
    let mut current_position = (0, 0);
    let mut grid = grid.lines().map(|l| {
        l.chars().map(|c|
            match c {
                '#' => Wall,
                '.' => Free,
                'O' => Boxx,
                '@' => Robot,
                _ => panic!("Unknown char")
            }
        ).collect_vec()
    }).collect_vec();
    let moves = moves.replace('\n', "").chars().map(|c|
        match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Unknown char")
        }
    ).collect_vec();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Robot {
                current_position = (j as i32, i as i32);
                break;
            }
        }
    }

    for move_ in moves {
        let mut next_position = (move_.0 + current_position.0, move_.1 + current_position.1);
        match grid[next_position.1  as usize][next_position.0  as usize] {
            Wall => next_position = current_position,
            Boxx => current_position = simulate_box_move(&mut grid, current_position, move_),
            Robot => panic!("two robots"),
            Free => {
                grid[current_position.1 as usize][current_position.0 as usize] = Free;
                current_position = next_position;
                grid[current_position.1 as usize][current_position.0 as usize] = Robot;
            },
        }
    }

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Boxx {
                sum += i * 100 + j;
            }
        }
    }

    sum.to_string()
}

fn simulate_box_move2(grid: &mut [Vec<Entity2>], current_position: (i32, i32), direction: (i32, i32)) -> bool {
    let part1 = current_position;
    let part2 = if grid[current_position.1 as usize][current_position.0 as usize] == Entity2::BoxL {
        (current_position.0 + 1, current_position.1)
    } else {
        (current_position.0 - 1, current_position.1)
    };

    if check_box_move2(grid, part1, direction) && check_box_move2(grid, part2, direction) {
        simulate_box_move2_part(grid, part1, direction);
        if direction == (0, -1) || direction == (0, 1) {
            simulate_box_move2_part(grid, part2, direction);
        }
        true
    } else {
        false
    }
}

fn simulate_box_move2_part(grid: &mut [Vec<Entity2>], current_position: (i32, i32), direction: (i32, i32)) {
    let next_position = (current_position.0 + direction.0, current_position.1 + direction.1);
    if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::Wall {
        return;
    } else if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::BoxL {
        simulate_box_move2_part(grid, next_position, direction);
        if direction == (0, -1) || direction == (0, 1) {
            simulate_box_move2_part(grid, (next_position.0 + 1, next_position.1), direction);
        }
    } else if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::BoxR {
        simulate_box_move2_part(grid, next_position, direction);
        if direction == (0, -1) || direction == (0, 1) {
            simulate_box_move2_part(grid, (next_position.0 - 1, next_position.1), direction);
        }
    }
    grid[next_position.1 as usize][next_position.0 as usize] = grid[current_position.1 as usize][current_position.0 as usize];
    grid[current_position.1 as usize][current_position.0 as usize] = Entity2::Free;
}

fn check_box_move2(grid: &[Vec<Entity2>], current_position: (i32, i32), direction: (i32, i32)) -> bool {
    let next_position = (current_position.0 + direction.0, current_position.1 + direction.1);
    if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::Wall {
        false
    } else if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::Free {
        true
    } else {
        let check1 = check_box_move2(grid, next_position, direction);
        if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::BoxL && (direction == (0, 1) || direction == (0, -1)) {
            check1 && check_box_move2(grid, (next_position.0 + 1, next_position.1), direction)
        } else if grid[next_position.1 as usize][next_position.0 as usize] == Entity2::BoxR && (direction == (0, 1) || direction == (0, -1)) {
            check1 && check_box_move2(grid, (next_position.0 - 1, next_position.1), direction)
        } else {
            check1
        }
    }
}

pub fn exec_day15_part2(input: &str) -> String {
    let input = input.trim();
    let (grid, moves) = input.split("\n\n").collect_tuple().unwrap();
    let mut current_position = (0, 0);
    let mut grid = grid.lines().map(|l| {
        l.chars().map(|c|
            match c {
                '#' => [Entity2::Wall, Entity2::Wall],
                '.' => [Entity2::Free, Entity2::Free],
                'O' => [Entity2::BoxL, Entity2::BoxR],
                '@' => [Entity2::Robot, Entity2::Free],
                _ => panic!("Unknown char")
            }
        ).flatten().collect_vec()
    }).collect_vec();
    let moves = moves.replace('\n', "").chars().map(|c|
        match c {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Unknown char")
        }
    ).collect_vec();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Entity2::Robot {
                current_position = (j as i32, i as i32);
                break;
            }
        }
    }

    for move_ in moves {
        let mut next_position = (move_.0 + current_position.0, move_.1 + current_position.1);
        match grid[next_position.1  as usize][next_position.0  as usize] {
            Entity2::Wall => continue,
            Entity2::BoxL | Entity2::BoxR => {
                if !simulate_box_move2(&mut grid, next_position, move_) {
                    continue;
                }
            },
            Entity2::Robot => panic!("two robots"),
            Entity2::Free => (),
        }
        grid[current_position.1 as usize][current_position.0 as usize] = Entity2::Free;
        current_position = next_position;
        grid[current_position.1 as usize][current_position.0 as usize] = Entity2::Robot;
    }

    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Entity2::BoxL {
                sum += i * 100 + j;
            }
        }
    }

    sum.to_string()
}

#[test]
fn test_day15_part1() {
    let input = match fs::read_to_string("./example/day15.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day15_part1(&input), "10092")
}

#[test]
fn test_day15_part2() {
    let input = match fs::read_to_string("./example/day15.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day15_part2(&input), "9021")
}
