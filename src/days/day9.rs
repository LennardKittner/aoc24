#[cfg(test)]
use std::fs;
use itertools::Itertools;


pub fn exec_day9_part1(input: &str) -> String {
    let mut input = input.bytes().map(|c| c - u8::try_from('0').unwrap()).collect_vec();
    let mut left = 0;
    let mut right = input.len()-1;
    let mut checksum = 0;
    let mut left_position = 0;

    while left < right {
        while input[left] == 0 {
            left += 1;
        }
        while input[right] == 0 {
            right -= 1;
        }
        let left_is_file = left % 2 == 0;
        let left_file_id = left / 2;
        let is_file_right = right % 2 == 0;
        let right_file_id = right / 2;

        if left_is_file {
            checksum += left_file_id * left_position;
            input[left] -= 1;
        } else if is_file_right {
            checksum += right_file_id * left_position;
            input[right] -= 1;
            input[left] -= 1;
        } else {
            input[right] -= 1;
            continue;
        }
        left_position += 1;
    }
    checksum.to_string()
}

pub fn exec_day9_part2(input: &str) -> String {
    let mut input = input.bytes().map(|c| (c - u8::try_from('0').unwrap(), 0u8)).collect_vec();
    let mut right = input.len()-1;
    let mut checksum = 0;
    let mut right_position: usize = 0;
    for (i, _) in input.iter() {
        right_position += *i as usize;
    }

    while 0 < right {
        let mut left_position: usize = 0;
        if right % 2 == 1 {
            right_position -= (input[right].0 + input[right].1) as usize;
            right -= 1;
            continue;
        }
        if input[right].0 == 0 {
            right -= 1;
            continue;
        }
        let mut file_moved = false;
        let right_file_id = right / 2;

        for left in 0..right {
            let left_is_file = left % 2 == 0;
            let rest_of_block = input[left].0 as usize;
            left_position += input[left].1 as usize;
            if !left_is_file && input[left].0 >= input[right].0 {
                file_moved = true;
                input[left].0 -= input[right].0;
                input[left].1 += input[right].0;
                for i in left_position..(left_position + input[right].0 as usize) {
                    checksum += right_file_id * i;
                }
                break;
            }
            left_position += rest_of_block;
        }
        if !file_moved {
            for i in (right_position-input[right].0 as usize)..right_position {
                checksum += right_file_id * i;
            }
        }
        right_position -= (input[right].0 + input[right].1) as usize;
        right -= 1;
    }
    checksum.to_string()
}

#[test]
fn test_day9_part1() {
    let input = match fs::read_to_string("./example/day9.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day9_part1(&input), "1928")
}

#[test]
fn test_day9_part2() {
    let input = match fs::read_to_string("./example/day9.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day9_part2(&input), "2858")
}
