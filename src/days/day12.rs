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

struct ExplorationReport {
    area: u64,
    perimeter_up: Vec<(usize, usize)>,
    perimeter_down: Vec<(usize, usize)>,
    perimeter_left: Vec<(usize, usize)>,
    perimeter_right: Vec<(usize, usize)>,
}

impl ExplorationReport {
    fn push_perimeter(&mut self, direction: Direction, value: (usize, usize)) {
        match direction {
            Up => self.perimeter_up.push(value),
            Down => self.perimeter_down.push(value),
            Left => self.perimeter_left.push(value),
            Right => self.perimeter_right.push(value),
        }
    }
}

#[allow(clippy::type_complexity)]
fn explore_region2(grid: &mut [Vec<(u8, bool)>], start: (usize, usize), char_of_region: u8, current_direction: Direction, report: &mut ExplorationReport) {
    if !(0..grid.len()).contains(&start.0) || !(0..grid[0].len()).contains(&start.1) {
        report.push_perimeter(current_direction, start);
        return;
    }
    if grid[start.0][start.1].0 != char_of_region {
        report.push_perimeter(current_direction, start);
        return;
    }
    if grid[start.0][start.1].1 {
        return;
    }
    grid[start.0][start.1].1 = true;

    explore_region2(grid, (start.0+1, start.1), char_of_region, Down, report);
    explore_region2(grid, (start.0.wrapping_sub(1), start.1), char_of_region, Up, report);
    explore_region2(grid, (start.0, start.1+1), char_of_region, Right, report);
    explore_region2(grid, (start.0, start.1.wrapping_sub(1)), char_of_region, Left, report);

    report.area += 1;
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

fn calc_sides(report: &mut ExplorationReport) -> u64 {
    if report.perimeter_up.is_empty() {
        return 0;
    }
    let perimeter_up = &mut report.perimeter_up;
    let perimeter_down = &mut report.perimeter_down;
    let perimeter_left = &mut report.perimeter_left;
    let perimeter_right = &mut report.perimeter_right;

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

    let mut result = count_horizontals(perimeter_up);
    result += count_horizontals(perimeter_down);
    result += count_verticals(perimeter_left);
    result += count_verticals(perimeter_right);

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
            let mut report = ExplorationReport {
                area: 0,
                perimeter_up: vec![],
                perimeter_down: vec![],
                perimeter_left: vec![],
                perimeter_right: vec![],
            };
            explore_region2(&mut grid, (i, j), c, Left, &mut report);
            result += report.area * calc_sides(&mut report);
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
