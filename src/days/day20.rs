use std::collections::{HashMap, VecDeque};
#[cfg(test)]
use std::fs;
use itertools::Itertools;
use crate::days::day20::State::{End, Free, Start, Visited, Wall};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Free,
    Start,
    End,
    Wall,
    Visited
}

fn check_shortcut(cheating_limit: usize, current_position: (usize, usize), end_position: (usize, usize), parents: &[Vec<Vec<(usize, usize)>>], distances: &[Vec<u32>], shortcuts: &mut HashMap<u32, u32>) {
    if current_position == end_position {
        return;
    }
    let own_distance = distances[current_position.0][current_position.1];

    for i in current_position.0.saturating_sub(cheating_limit)..=(current_position.0+cheating_limit) {
        for j in current_position.1.saturating_sub(cheating_limit)..=(current_position.1+cheating_limit) {
            let distance_to_other = current_position.0.abs_diff(i) + current_position.1.abs_diff(j);
            if distance_to_other > cheating_limit {
                continue;
            }
            check_direction(parents, distances, shortcuts, own_distance, (i, j), distance_to_other);
        }
    }

    for parent in &parents[current_position.0][current_position.1] {
        check_shortcut(cheating_limit, *parent, end_position, parents, distances, shortcuts);
    }
}

fn check_direction(parents: &[Vec<Vec<(usize, usize)>>], distances: &[Vec<u32>], shortcuts: &mut HashMap<u32, u32>, own_distance: u32, other: (usize, usize), distance_to_other: usize) {
    if parents.len() > other.0 && parents[0].len() > other.1 {
        let distance = distances[other.0][other.1];
        let new_distance = own_distance as i64 - distance as i64 - distance_to_other as i64;
        if new_distance > 0 {
            let new_distance = new_distance as u32;
            let mut new_count = 1;
            if shortcuts.contains_key(&new_distance) {
                new_count += shortcuts[&new_distance];
            }
            shortcuts.insert(new_distance, new_count);
        }
    }
}

pub fn exec_day20_part1(input: &str) -> String {
    let (parents, distances, end_position, start_position) = parse_input(input);
    let mut shortcuts= HashMap::new();
    check_shortcut(2, start_position, end_position, &parents, &distances, &mut shortcuts);

    #[cfg(test)] const THRESHOLD: u32 = 0;
    #[cfg(not(test))] const THRESHOLD: u32 = 100;
    let mut result = 0;
    for (saved, number_of_times) in shortcuts {
        if saved >= THRESHOLD {
            result += number_of_times;
        }
    }
    result.to_string()
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<Vec<Vec<(usize, usize)>>>, Vec<Vec<u32>>, (usize, usize), (usize, usize)) {
    let mut grid = input.trim().lines().map(|l|
        l.bytes().map(|c| {
            match c {
                b'.' => Free,
                b'#' => Wall,
                b'S' => Start,
                b'E' => End,
                _ => panic!("unknown char")
            }
        }).collect_vec()).collect_vec();
    let mut parents = Vec::new();
    parents.reserve_exact(grid.len());
    let mut distances = Vec::new();
    distances.reserve_exact(grid.len());
    let mut current_level_queue = VecDeque::new();
    let mut next_level_queue = VecDeque::new();
    let mut end_position = (0, 0);
    let mut start_position = (0, 0);
    for i in 0..grid.len() {
        parents.push(Vec::new());
        distances.push(Vec::new());
        for j in 0..grid[i].len() {
            parents[i].push(vec![]);
            distances[i].push(u32::MAX);
            if grid[i][j] == End {
                distances[i][j] = 0;
                current_level_queue.push_back((i, j));
                grid[i][j] = Visited;
                end_position = (i, j);
            }
            if grid[i][j] == Start {
                start_position = (i, j);
            }
        }
    }
    let mut current_distance = 1;
    while !current_level_queue.is_empty() || !next_level_queue.is_empty() {
        if current_level_queue.is_empty() {
            current_level_queue = next_level_queue;
            current_distance += 1;
            next_level_queue = VecDeque::new();
        }
        let (x, y) = current_level_queue.pop_front().unwrap();

        if distances[x + 1][y] > current_distance {
            parents[x + 1][y].push((x, y));
        }
        if grid[x + 1][y] != Visited && grid[x + 1][y] != Wall {
            grid[x + 1][y] = Visited;
            distances[x + 1][y] = current_distance;
            next_level_queue.push_back((x + 1, y))
        }
        if distances[x - 1][y] > current_distance {
            parents[x - 1][y].push((x, y));
        }
        if grid[x - 1][y] != Visited && grid[x - 1][y] != Wall {
            grid[x - 1][y] = Visited;
            distances[x - 1][y] = current_distance;
            next_level_queue.push_back((x - 1, y))
        }
        if distances[x][y + 1] > current_distance {
            parents[x][y + 1].push((x, y));
        }
        if grid[x][y + 1] != Visited && grid[x][y + 1] != Wall {
            grid[x][y + 1] = Visited;
            distances[x][y + 1] = current_distance;
            next_level_queue.push_back((x, y + 1))
        }
        if distances[x][y - 1] > current_distance {
            parents[x][y - 1].push((x, y));
        }
        if grid[x][y - 1] != Visited && grid[x][y - 1] != Wall {
            grid[x][y - 1] = Visited;
            distances[x][y - 1] = current_distance;
            next_level_queue.push_back((x, y - 1))
        }
    }
    (parents, distances, end_position, start_position)
}

pub fn exec_day20_part2(input: &str) -> String {
    let (parents, distances, end_position, start_position) = parse_input(input);
    let mut shortcuts= HashMap::new();
    check_shortcut(20, start_position, end_position, &parents, &distances, &mut shortcuts);

    #[cfg(test)] const THRESHOLD: u32 = 50;
    #[cfg(not(test))] const THRESHOLD: u32 = 100;
    let mut result = 0;
    for (saved, number_of_times) in shortcuts {
        if saved >= THRESHOLD {
            result += number_of_times;
        }
    }
    result.to_string()
}

#[test]
fn test_day20_part1() {
    let input = match fs::read_to_string("./example/day20.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day20_part1(&input), "44")
}

#[test]
fn test_day20_part2() {
    let input = match fs::read_to_string("./example/day20.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day20_part2(&input), "285")
}
