use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::Vacant;
#[cfg(test)]
use std::fs;
use std::ops::Range;
use itertools::Itertools;

fn calc_antinode_location(n1: (usize, usize), n2: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    let mut location1 = (0, 0);
    let mut location2 = (0, 0);
    location1.0 = (2 * n2.0).wrapping_sub(n1.0);
    location1.1 = (2 * n2.1).wrapping_sub(n1.1);

    location2.0 = (2 * n1.0).wrapping_sub(n2.0);
    location2.1 = (2 * n1.1).wrapping_sub(n2.1);

    (location1, location2)
}

fn calc_antinode_location2(n1: (usize, usize), n2: (usize, usize), bound0: Range<usize>, bound1: Range<usize>) -> Vec<(usize, usize)> {
    let delta1 = (n1.0 as i32 - n2.0 as i32, n1.1 as i32 - n2.1 as i32);
    let delta2 = (n2.0 as i32 - n1.0 as i32, n2.1 as i32 - n1.1 as i32);
    let mut result = Vec::new();

    let mut current = (n1.0 as i32, n1.1 as i32);
    while bound0.contains(&(current.0 as usize)) && bound1.contains(&(current.1 as usize)) {
        result.push((current.0 as usize, current.1 as usize));
        current.0 += delta1.0;
        current.1 += delta1.1;
    }

    let mut current = (n2.0 as i32, n2.1 as i32);
    while bound0.contains(&(current.0 as usize)) && bound1.contains(&(current.1 as usize)) {
        result.push((current.0 as usize, current.1 as usize));
        current.0 += delta2.0;
        current.1 += delta2.1;
    }

    result
}

pub fn exec_day8_part1(input: &str) -> String {
    let (grid, antennas) = parse_input(input);
    let antinode_locations: HashSet<(usize, usize)> = antennas.values().flat_map(|antenas|
        antenas.iter().combinations(2).filter_map(|antenas| {
            let location = calc_antinode_location(*antenas[0], *antenas[1]);
            let mut result = Vec::new();
            if (0..grid.len()).contains(&location.0.0) && (0..grid[0].len()).contains(&location.0.1) {
                result.push(location.0);
            }
            if (0..grid.len()).contains(&location.1.0) && (0..grid[0].len()).contains(&location.1.1) {
                result.push(location.1);
            }
            if !result.is_empty() {
                Some(result)
            } else {
                None
            }
        })
            .flatten()
            .collect_vec()
    )
        .collect();

    antinode_locations.len().to_string()
}

pub fn exec_day8_part2(input: &str) -> String {
    let (grid, antennas) = parse_input(input);
    let antinode_locations: HashSet<(usize, usize)> = antennas.values().flat_map(|antenas|
        antenas.iter().combinations(2).filter_map(|antenas| {
            let result = calc_antinode_location2(*antenas[0], *antenas[1], 0..grid.len(), 0..grid[0].len());
            if !result.is_empty() {
                Some(result)
            } else {
                None
            }
        })
            .flatten()
            .collect_vec()
    )
        .collect();

    antinode_locations.len().to_string()
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<Vec<u8>>, HashMap<u8, Vec<(usize, usize)>>) {
    let grid = input.lines().map(|l| l.bytes().collect_vec()).collect_vec();
    let mut antenas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != u8::try_from('.').unwrap() {
                if let Vacant(e) = antenas.entry(grid[i][j]) {
                    e.insert(vec![(i, j)]);
                } else {
                    antenas.get_mut(&grid[i][j]).unwrap().push((i, j));
                }
            }
        }
    }
    (grid, antenas)
}

#[test]
fn test_day8_part1() {
    let input = match fs::read_to_string("./example/day8.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day8_part1(&input), "14")
}

#[test]
fn test_day8_part2() {
    let input = match fs::read_to_string("./example/day8.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day8_part2(&input), "34")
}
