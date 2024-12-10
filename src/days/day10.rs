use std::collections::HashSet;
#[cfg(test)]
use std::fs;
use itertools::Itertools;


fn explore(position: (usize, usize), grid: &[Vec<u8>], previous_height: i32) -> Vec<(usize, usize)> {
    if !(0..grid.len()).contains(&position.0) || !(0..grid[0].len()).contains(&position.1) {
        return Vec::new();
    }
    if previous_height + 1 != grid[position.0][position.1] as i32 {
        return Vec::new();
    }
    if grid[position.0][position.1] == 9 {
        return vec![position];
    }
    let height = grid[position.0][position.1] as i32;
    let mut result = explore((position.0 + 1, position.1), grid,  height);
    result.append(&mut explore((position.0.wrapping_sub(1), position.1), grid, height));
    result.append(&mut explore((position.0, position.1 + 1), grid, height));
    result.append(&mut explore((position.0, position.1.wrapping_sub(1)), grid, height));

    result
}

pub fn exec_day10_part1(input: &str) -> String {
    let (grid, start_positions) = parse_input(input);
    start_positions.iter().map(|&pos| {
        let h : HashSet<(usize, usize)> = HashSet::from_iter(explore(pos, &grid, -1).iter().copied());
        h.len()
    }).sum::<usize>().to_string()
}

pub fn exec_day10_part2(input: &str) -> String {
    let (grid, start_positions) = parse_input(input);
    start_positions.iter().map(|&pos| {
        explore(pos, &grid, -1).len()
    }).sum::<usize>().to_string()
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<(usize, usize)>) {
    let grid = input.lines().map(|l| l.bytes().map(|b| if b != 0x2e {
        b - u8::try_from('0').unwrap()
    } else {
        99
    }).collect_vec()).collect_vec();
    let mut start_positions = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                start_positions.push((i, j));
            }
        }
    }
    (grid, start_positions)
}

#[test]
fn test_day10_part1() {
    let input = match fs::read_to_string("./example/day10.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day10_part1(&input), "36")
}

#[test]
fn test_day10_part2() {
    let input = match fs::read_to_string("./example/day10.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day10_part2(&input), "81")
}
