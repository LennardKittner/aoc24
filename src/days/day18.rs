use std::collections::{HashMap, VecDeque};
#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

#[cfg(test)] const GRID_LENGTH: usize = 6+1;
#[cfg(not(test))] const GRID_LENGTH: usize = 70+1;
#[cfg(test)] const NUM_BYTES: usize = 12;
#[cfg(not(test))] const NUM_BYTES: usize = 1024;


pub fn exec_day18_part1(input: &str) -> String {
    let byte_locations = input.trim().lines().map(|l| l.split(',').map(|n| u32::from_str(n).unwrap()).collect_tuple::<(u32, u32)>().unwrap()).collect_vec();
    let mut grid = [[false; GRID_LENGTH]; GRID_LENGTH];
    for location in byte_locations[0..NUM_BYTES].iter() {
        grid[location.0 as usize][location.1 as usize] = true;
    }
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize));
    let mut parent = HashMap::new();
    grid[0][0] = true;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if x == GRID_LENGTH-1 && y == GRID_LENGTH-1 {
            break;
        }
        if (0..GRID_LENGTH).contains(&(x + 1)) && !grid[x+1][y] {
            parent.insert((x+1, y), (x, y));
            grid[x+1][y] = true;
            queue.push_back((x+1, y))
        }
        if x > 0 && !grid[x-1][y] {
            parent.insert((x-1, y), (x, y));
            grid[x-1][y] = true;
            queue.push_back((x-1, y))
        }
        if (0..GRID_LENGTH).contains(&(y + 1)) && !grid[x][y+1] {
            parent.insert((x, y+1), (x, y));
            grid[x][y+1] = true;
            queue.push_back((x, y+1))
        }
        if y > 0 && !grid[x][y-1] {
            parent.insert((x, y-1), (x, y));
            grid[x][y-1] = true;
            queue.push_back((x, y-1))
        }
    }
    let mut current = (GRID_LENGTH-1, GRID_LENGTH-1);
    let mut len = 0;
    while current != (0usize, 0usize) {
        current = parent[&current];
        len += 1;
    }

    len.to_string()
}

pub fn exec_day18_part2(input: &str) -> String {
    let byte_locations = input.trim().lines().map(|l| l.split(',').map(|n| u32::from_str(n).unwrap()).collect_tuple::<(u32, u32)>().unwrap()).collect_vec();
    let grid = [[false; GRID_LENGTH]; GRID_LENGTH];

    let mut l = 0;
    let mut r = byte_locations.len();
    while l < r {
        if test_until(l + (r-l)/2, &byte_locations, &grid) {
            l = l + (r-l)/2+1;
        } else {
            r = l + (r-l)/2-1;
        }
    };

    format!("{},{}", byte_locations[l].0, byte_locations[l].1)
}

fn test_until(num_bytes: usize, byte_locations: &[(u32, u32)], grid: &[[bool; GRID_LENGTH]; GRID_LENGTH]) -> bool {
    let mut grid = *grid;
    for location in byte_locations[0..=num_bytes].iter() {
        grid[location.0 as usize][location.1 as usize] = true;
    }
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize));
    grid[0][0] = true;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if x == GRID_LENGTH - 1 && y == GRID_LENGTH - 1 {
            return true;
        }
        if (0..GRID_LENGTH).contains(&(x + 1)) && !grid[x + 1][y] {
            grid[x + 1][y] = true;
            queue.push_back((x + 1, y))
        }
        if x > 0 && !grid[x - 1][y] {
            grid[x - 1][y] = true;
            queue.push_back((x - 1, y))
        }
        if (0..GRID_LENGTH).contains(&(y + 1)) && !grid[x][y + 1] {
            grid[x][y + 1] = true;
            queue.push_back((x, y + 1))
        }
        if y > 0 && !grid[x][y - 1] {
            grid[x][y - 1] = true;
            queue.push_back((x, y - 1))
        }
    }
    false
}

#[test]
fn test_day18_part1() {
    let input = match fs::read_to_string("./example/day18.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day18_part1(&input), "22")
}

#[test]
fn test_day18_part2() {
    let input = match fs::read_to_string("./example/day18.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day18_part2(&input), "6,1")
}
