use std::cmp::min;
use itertools::Itertools;


fn check_string(input: &[u8]) -> i32 {
    let xmas: [u8; 4] = [u8::try_from('X').unwrap(), u8::try_from('M').unwrap(), u8::try_from('A').unwrap(), u8::try_from('S').unwrap()];
    let mut forwards = 0;
    let mut backwards = 0;
    let mut count = 0;
    for &c in input {
        if c == xmas[forwards] {
            forwards += 1;
        } else if c == xmas[0] {
            forwards = 1;
        } else {
            forwards = 0;
        }
        if c == xmas[3 - backwards] {
            backwards += 1;
        } else if c == xmas[3] {
            backwards = 1;
        } else {
            backwards = 0;
        }
        if forwards == 4 {
            count += 1;
            forwards = 0;
        }
        if backwards == 4 {
            count += 1;
            backwards = 0;
        }
    }
    count
}

pub fn exec_day4_part1(input: &str) -> String {
    //Horizontals
    let mut count = check_string(input.as_bytes());
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();

    //Vertical
    let mut columns = Vec::new();
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            columns.push(grid[j][i]);
        }
        columns.push(0);
    }
    count += check_string(&columns);

    //Diagonal right
    let mut diagonals = Vec::new();
    for j in 0..grid[0].len() {
        for i in 0..min(grid[0].len()-j, grid.len()) {
            diagonals.push(grid[i][i+j])
        }
        diagonals.push(0);
    }
    for j in 1..grid.len() {
        for i in 0..min(grid[0].len(), grid.len()-j) {
            diagonals.push(grid[j+i][i]);
        }
        diagonals.push(0);
    }
    count += check_string(&diagonals);

    //Diagonal left
    diagonals.clear();
    for j in 0..grid[0].len() {
        for i in 0..min(j+1, grid.len()) {
            diagonals.push(grid[i][j-i])
        }
        diagonals.push(0);
    }
    for j in 1..grid.len() {
        for i in 0..min(grid[0].len(), grid.len()-j) {
            diagonals.push(grid[j+i][grid[0].len()-1-i]);
        }
        diagonals.push(0);
    }
    count += check_string(&diagonals);

    count.to_string()
}

fn contains_x_mas(z1: &[u8], z2: &[u8], z3: &[u8]) -> bool {
    (z1[0] == u8::try_from('M').unwrap() &&
    z2[1] == u8::try_from('A').unwrap() &&
    z3[2] == u8::try_from('S').unwrap()
        ||
    z1[0] == u8::try_from('S').unwrap() &&
    z2[1] == u8::try_from('A').unwrap() &&
    z3[2] == u8::try_from('M').unwrap())
    &&
    (z1[2] == u8::try_from('M').unwrap() &&
    z2[1] == u8::try_from('A').unwrap() &&
    z3[0] == u8::try_from('S').unwrap()
        ||
    z1[2] == u8::try_from('S').unwrap() &&
    z2[1] == u8::try_from('A').unwrap() &&
    z3[0] == u8::try_from('M').unwrap())
}

pub fn exec_day4_part2(input: &str) -> String {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();

    let mut count = 0;
    for i in 0..grid.len()-2 {
        for j in 0..grid.len()-2 {
            let z1 = &grid[i][j..(j + 3)];
            let z2 = &grid[i+1][j..(j + 3)];
            let z3 = &grid[i+2][j..(j + 3)];
            if contains_x_mas(z1, z2, z3) {
                count += 1;
            }
        }
    }
    count.to_string()
}
