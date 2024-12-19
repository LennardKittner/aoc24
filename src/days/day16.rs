use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};
#[cfg(test)]
use std::fs;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::days::day16::Entry::{End, Free, Start, Wall};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Entry {
    Free,
    Wall,
    Start,
    End
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

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Entry>]) {
    let string = grid.iter().map(|l| l.iter().map(|e|
        match e {
            Free => '.',
            Wall => '#',
            Start => 'S',
            End => 'E',
        }).collect::<String>()
    ).join("\n");
    println!("{string}");
}

fn calc_priority(value: i64) -> Reverse<i64> {
    Reverse(value)
}

pub fn exec_day16_part1(input: &str) -> String {
    let grid = input.trim().lines().map(|l|
        l.bytes().map(|c|
            match c {
                b'#' => Wall,
                b'S' => Start,
                b'.' => Free,
                b'E' => End,
                _ => panic!("unknown char"),
            }).collect_vec()).collect_vec();

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
        if current_position == end_location {
            break;
        }
        let current_cost = *distances.get(&(current_position, direction)).unwrap();

        let dir_costs = [(direction, 1), (turn_right(direction), 1001), (turn_right(turn_right(direction)), 2001), (turn_right(turn_right(turn_right(direction))), 1001)];
        for (dir, cost) in dir_costs {
            let neighbor = ((current_position.0 + dir.0 as i64, current_position.1 + dir.1 as i64), dir);
            if grid[neighbor.0.0 as usize][neighbor.0.1 as usize] == Wall {
                continue;
            }
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

#[allow(clippy::type_complexity)]
fn count_parents(current_node: ((i64, i64), (i8, i8)), parents: &HashMap<((i64, i64), (i8, i8)), Vec<((i64, i64), (i8, i8))>>, nodes: &mut HashSet<(i64, i64)>) {
    nodes.insert(current_node.0);
    if !parents.contains_key(&current_node) {
        return;
    }
    for parent in parents[&current_node].iter() {
        count_parents(*parent, parents, nodes);
    }
}

pub fn exec_day16_part2(input: &str) -> String {
    let grid = input.trim().lines().map(|l|
        l.bytes().map(|c|
            match c {
                b'#' => Wall,
                b'S' => Start,
                b'.' => Free,
                b'E' => End,
                _ => panic!("unknown char"),
            }).collect_vec()).collect_vec();

    let mut prio_queue = PriorityQueue::new();
    let mut distances = HashMap::new();
    let mut end_location = (0, 0);
    let mut paths = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            paths.insert(((i as i64, j as i64), (0i8, 1i8)), Vec::new());
            paths.insert(((i as i64, j as i64), (0i8, -1i8)), Vec::new());
            paths.insert(((i as i64, j as i64), (1i8, 0i8)), Vec::new());
            paths.insert(((i as i64, j as i64), (-1i8, 0i8)), Vec::new());
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
        if current_position == end_location {
            break;
        }
        let current_cost = *distances.get(&(current_position, direction)).unwrap();

        let dir_costs = [(direction, 1), (turn_right(direction), 1001), (turn_right(turn_right(direction)), 2001), (turn_right(turn_right(turn_right(direction))), 1001)];
        for (dir, cost) in dir_costs {
            let neighbor = ((current_position.0 + dir.0 as i64, current_position.1 + dir.1 as i64), dir);
            if grid[neighbor.0.0 as usize][neighbor.0.1 as usize] == Wall {
                continue;
            }
            if let Some(&neighbor_cost) = distances.get(&neighbor) {
                match neighbor_cost.cmp(&(current_cost + cost)) {
                    Ordering::Less => (),
                    Ordering::Equal => paths.get_mut(&neighbor).unwrap().push((current_position, direction)),
                    Ordering::Greater => {
                        paths.insert(neighbor, vec![(current_position, direction)]);
                        distances.insert(neighbor, current_cost + cost);
                        prio_queue.push_increase(neighbor, calc_priority(current_cost + cost));
                    }
                }
            }
        }
    }

    let mut nodes = HashSet::new();
    count_parents((end_location, (0, 1)), &paths, &mut nodes);
    count_parents((end_location, (0, -1)), &paths, &mut nodes);
    count_parents((end_location, (1, 0)), &paths, &mut nodes);
    count_parents((end_location, (-1, 0)), &paths, &mut nodes);
    nodes.len().to_string()
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
    assert_eq!(exec_day16_part2(&input), "64")
}
