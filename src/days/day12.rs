use std::cmp::Ordering;
#[cfg(test)]
use std::fs;
use itertools::Itertools;
use crate::days::day12::Direction::{Down, Left, Right, Up};

fn explore_region(grid: &mut [Vec<(u8, bool)>], start: (usize, usize), char_of_region: u8) -> (u64, u64) {
    if !(0..grid.len()).contains(&start.0) || !(0..grid[0].len()).contains(&start.1) {
        return (0, 1);
    }
    if grid[start.0][start.1].0 != char_of_region {
        return (0, 1);
    }
    if grid[start.0][start.1].1 {
        return (0, 0);
    }
    grid[start.0][start.1].1 = true;

    let right = explore_region(grid, (start.0+1, start.1), char_of_region);
    let left = explore_region(grid, (start.0.wrapping_sub(1), start.1), char_of_region);
    let up = explore_region(grid, (start.0, start.1+1), char_of_region);
    let down = explore_region(grid, (start.0, start.1.wrapping_sub(1)), char_of_region);

    (1 + right.0 + left.0 + up.0 + down.0, right.1 + left.1 + up.1 + down.1)
}

pub fn exec_day12_part1(input: &str) -> String {
    let mut grid = input.lines().map(|l| l.bytes().map(|c| {
        (c, false)
    }).collect_vec()).collect_vec();
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let char = grid[i][j].0;
            let (area, perimeter) = explore_region(&mut grid, (i, j), char);
            result += area * perimeter;
        }
    }
    result.to_string()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[allow(clippy::type_complexity)]
fn explore_region2(grid: &mut [Vec<(u8, bool)>], start: (usize, usize), char_of_region: u8, current_direction: Direction) -> (u64, Vec<((usize, usize), Direction)>) {
    if !(0..grid.len()).contains(&start.0) || !(0..grid[0].len()).contains(&start.1) {
        return (0, vec![(start, current_direction)]);
    }
    if grid[start.0][start.1].0 != char_of_region {
        return (0, vec![(start, current_direction)]);
    }
    if grid[start.0][start.1].1 {
        return (0, vec![]);
    }
    grid[start.0][start.1].1 = true;

    let mut right = explore_region2(grid, (start.0+1, start.1), char_of_region, Down);
    let mut left = explore_region2(grid, (start.0.wrapping_sub(1), start.1), char_of_region, Up);
    let mut up = explore_region2(grid, (start.0, start.1+1), char_of_region, Right);
    let mut down = explore_region2(grid, (start.0, start.1.wrapping_sub(1)), char_of_region, Left);

    let mut perimeter = Vec::new();
    perimeter.append(&mut right.1);
    perimeter.append(&mut left.1);
    perimeter.append(&mut up.1);
    perimeter.append(&mut down.1);

    (1 + right.0 + left.0 + up.0 + down.0, perimeter)
}

fn count_horizontals(perimeter_x: &[(usize, usize)]) -> u64 {
    let mut result = 1;
    for i in 1..perimeter_x.len() {
        if perimeter_x[i - 1].1.abs_diff(perimeter_x[i].1) > 1 || perimeter_x[i].0 != perimeter_x[i - 1].0 {
            result += 1;
        }
    }
    result
}

fn count_verticals(perimeter_y: &[(usize, usize)]) -> u64 {
    let mut result = 1;
    for i in 1..perimeter_y.len() {
        if perimeter_y[i-1].0.abs_diff(perimeter_y[i].0) > 1 || perimeter_y[i].1 != perimeter_y[i-1].1 {
            result += 1;
        }
    }
    result
}

fn calc_sides(perimeter: &[((usize, usize), Direction)]) -> u64 {
    if perimeter.is_empty() {
        return 0;
    }
    let mut perimeter_up = Vec::new();
    let mut perimeter_down = Vec::new();
    let mut perimeter_left = Vec::new();
    let mut perimeter_right = Vec::new();

    for e in perimeter {
        match e.1 {
            Up => perimeter_up.push(e.0),
            Down => perimeter_down.push(e.0),
            Left => perimeter_left.push(e.0),
            Right => perimeter_right.push(e.0),
        }
    }

    let sort_routine = |a: &(usize, usize), b: &(usize, usize)| {
        if a.1 > b.1 {
            Ordering::Greater
        } else if a.1 < b.1 {
            Ordering::Less
        } else if a.0 > b.0 {
            Ordering::Greater
        } else if a.0 < b.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };

    perimeter_up.sort();
    perimeter_down.sort();
    perimeter_left.sort_by(sort_routine);
    perimeter_right.sort_by(sort_routine);

    let mut result = count_horizontals(&perimeter_up);
    result += count_horizontals(&perimeter_down);
    result += count_verticals(&perimeter_left);
    result += count_verticals(&perimeter_right);

    result
}

pub fn exec_day12_part2(input: &str) -> String {
    let mut grid = input.lines().map(|l| l.bytes().map(|c| {
        (c, false)
    }).collect_vec()).collect_vec();
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j].0;
            let (area, perimeter) = explore_region2(&mut grid, (i, j), c, Left);
            result += area * calc_sides(&perimeter);
        }
    }
    result.to_string()
}

#[test]
fn test_day12_part1() {
    let input = match fs::read_to_string("./example/day12.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day12_part1(&input), "1930")
}

#[test]
fn test_day12_part2() {
    let input = match fs::read_to_string("./example/day12.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day12_part2(&input), "1206")
}
