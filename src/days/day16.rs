use std::cmp::Reverse;
use std::collections::HashMap;
#[cfg(test)]
use std::fs;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::days::day16::Entry::{End, Free, Start, Visited, Wall};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Entry {
    Free,
    Wall,
    Start,
    End,
    Visited
}

fn turn_right(direction: (i8, i8)) -> (i8, i8) {
    match direction {
        (1, 0) => (0, -1),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (0, -1) => (-1, 0),
        _ => panic!("invalid direction")
    }
}

fn print_grid(grid: &[Vec<Entry>]) {
    let string = grid.iter().map(|l| l.iter().map(|e|
        match e {
            Free => '.',
            Wall => '#',
            Start => 'S',
            End => 'E',
            Visited => '*',
        }).collect::<String>()
    ).join("\n");
    println!("{string}");
}

fn find_best_path(grid: &mut [Vec<Entry>], current_position: (i64, i64), direction: (i8, i8), cache: &mut HashMap<((i64, i64), (i8, i8)), u64>) -> u64 {
    let x = 10;
    let y = 4;
    if current_position.0 == y && current_position.1 == x {
        println!("{current_position:?} {direction:?}");
        print_grid(grid);
        println!("hi");
    }
    if let Some(result) = cache.get(&(current_position, direction)) {
        if current_position.0 == y && current_position.1 == x {
            println!("using cache {result}");
        }
        return *result;
    }
    if grid[current_position.0 as usize][current_position.1 as usize] == Wall {
        return u64::MAX / 2; // avoid overflows
    }
    if grid[current_position.0 as usize][current_position.1 as usize] == End {
        return 0;
    }
    if grid[current_position.0 as usize][current_position.1 as usize] == Visited {
        return u64::MAX / 2; // avoid overflows
    }

    grid[current_position.0 as usize][current_position.1 as usize] = Visited;

    let dir1 = direction;
    let dir2 = turn_right(dir1);
    let dir3 = turn_right(dir2);
    let dir4 = turn_right(dir3);

    let distances = [
        find_best_path(grid, (current_position.0 + dir1.0 as i64, current_position.1 + dir1.1 as i64), dir1, cache) + 1,
        find_best_path(grid, (current_position.0 + dir2.0 as i64, current_position.1 + dir2.1 as i64), dir2, cache) + 1001,
        find_best_path(grid, (current_position.0 + dir3.0 as i64, current_position.1 + dir3.1 as i64), dir3, cache) + 2001,
        find_best_path(grid, (current_position.0 + dir4.0 as i64, current_position.1 + dir4.1 as i64), dir4, cache) + 1001,
    ];

    if current_position.0 == y && current_position.1 == x {
        println!("{distances:?}");
    }

    let distance = *distances.iter().min().unwrap();

    grid[current_position.0 as usize][current_position.1 as usize] = Free;
    cache.insert((current_position, direction), distance);
    distance
}

fn calc_priority(value: i64) -> Reverse<i64> {
    Reverse(value)
}

pub fn exec_day16_part1(input: &str) -> String {
    let mut cache: HashMap<((i64, i64), (i8, i8)), u64> = HashMap::new();
    let mut grid = input.trim().lines().map(|l|
        l.bytes().map(|c|
            match c {
                b'#' => Wall,
                b'S' => Start,
                b'.' => Free,
                b'E' => End,
                _ => panic!("unknown char"),
            }).collect_vec()).collect_vec();


    // let mut result = 0;
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         if grid[i][j] == Start {
    //             result = find_best_path(&mut grid, (i as i64, j as i64), (0, 1), &mut cache);
    //             break;
    //         }
    //     }
    // }
    // result.to_string()

    let mut prio_queue = PriorityQueue::new();
    let mut distances = HashMap::new();
    let mut end_location = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Start {
                prio_queue.push(((i as i64, j as i64), (0i8, 1i8)), calc_priority(0));
                prio_queue.push(((i as i64, j as i64), (0i8, -1i8)), calc_priority(2000));
                prio_queue.push(((i as i64, j as i64), (1i8, 0i8)), calc_priority(1000));
                prio_queue.push(((i as i64, j as i64), (-1i8, 0i8)), calc_priority(1000));
                distances.insert(((i as i64, j as i64), (0i8, 1i8)), 0);
                distances.insert(((i as i64, j as i64), (0i8, -1i8)), 2000);
                distances.insert(((i as i64, j as i64), (1i8, 0i8)), 1000);
                distances.insert(((i as i64, j as i64), (-1i8, 0i8)), 1000);
            } else {
                distances.insert(((i as i64, j as i64), (0i8, 1i8)), i64::MAX);
                distances.insert(((i as i64, j as i64), (0i8, -1i8)), i64::MAX);
                distances.insert(((i as i64, j as i64), (1i8, 0i8)), i64::MAX);
                distances.insert(((i as i64, j as i64), (-1i8, 0i8)), i64::MAX);
            }
            if grid[i][j] == End {
                end_location = (i as i64, j as i64);
            }
        }
    }
    while !prio_queue.is_empty() {
        let ((current_position, direction), _) = prio_queue.pop().unwrap();
        let current_cost = *distances.get(&(current_position, direction)).unwrap();

        let dir_costs = [(direction, 1), (turn_right(direction), 1001), (turn_right(turn_right(direction)), 2001), (turn_right(turn_right(turn_right(direction))), 1001)];
        for (dir, cost) in dir_costs {
            let neighbor = ((current_position.0 + dir.0 as i64, current_position.1 + dir.1 as i64), dir);
            if let Some(&neighbor_cost) = distances.get(&neighbor) {
                if neighbor_cost > current_cost + cost {
                    distances.insert(neighbor, current_cost + cost);
                    prio_queue.push_increase(neighbor, calc_priority(current_cost + cost));
                }
            }
        }
    }
    [
        distances.get(&(end_location, (0, 1))).unwrap(),
        distances.get(&(end_location, (0, -1))).unwrap(),
        distances.get(&(end_location, (1, 0))).unwrap(),
        distances.get(&(end_location, (-1, 0))).unwrap()
    ].iter().min().unwrap().to_string()
}

pub fn exec_day16_part2(input: &str) -> String {
    //TODO
    "Not implemented!".to_string()
}

#[test]
fn test_day16_part1() {
    let input = match fs::read_to_string("./example/day16.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day16_part1(&input), "11048")
}

#[test]
fn test_day16_part2() {
    let input = match fs::read_to_string("./example/day16.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day16_part2(&input), "TODO")
}
