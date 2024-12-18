#[cfg(test)]
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use crate::days::day17::Opcode::{ADV, BDV, BST, BXC, BXL, CDV, JNZ, OUT};
use crate::days::day17::OperantType::{Combo, Literal};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct CPU {
    program_counter: u64,
    a: u64,
    b: u64,
    c: u64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum OperantType {
    Literal,
    Combo
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Opcode {
    fn get_opcode(cpu: &CPU, program: &[u8]) -> Self {
        match program[cpu.program_counter as usize] {
            0 => ADV,
            1 => BXL,
            2 => BST,
            3 => JNZ,
            4 => BXC,
            5 => OUT,
            6 => BDV,
            7 => CDV,
            _ => panic!("invalid opcode"),
        }
    }

    fn get_type(&self) -> OperantType {
        match self {
            ADV => Combo,
            BXL => Literal,
            BST => Combo,
            JNZ => Literal,
            BXC => Literal,
            OUT => Combo,
            BDV => Combo,
            CDV => Combo,
        }
    }

    fn exec(cpu: &mut CPU, program: &[u8]) -> Option<u64> {
        let opcode = Self::get_opcode(cpu, program);
        let operant = opcode.get_type().get_operant(cpu, program);
        let mut next_instruction_at = cpu.program_counter + 2;
        let mut output = None;
        match opcode {
            ADV => cpu.a /= 2u64.pow(operant as u32),
            BXL => cpu.b ^= operant,
            BST => cpu.b = operant % 8,
            JNZ => if cpu.a != 0 { next_instruction_at = operant },
            BXC => cpu.b ^= cpu.c,
            OUT => output = Some(operant % 8),
            BDV => cpu.b = cpu.a / 2u64.pow(operant as u32),
            CDV => cpu.c = cpu.a / 2u64.pow(operant as u32),
        }
        cpu.program_counter = next_instruction_at;
        output
    }
}

impl OperantType {
    fn get_operant(&self, cpu: &CPU, program: &[u8]) -> u64 {
        match self {
            Literal => program[cpu.program_counter as usize + 1] as u64,
            Combo => {
                match program[cpu.program_counter as usize + 1] {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => cpu.a,
                    5 => cpu.b,
                    6 => cpu.c,
                    _ => panic!("invalid operant"),
                }
            }
        }
    }
}


pub fn exec_day17_part1(input: &str) -> String {
    let (registers, program) = input.trim().split("\n\n").collect_tuple().unwrap();
    let (a, b, c) = registers.lines().map(|l| u64::from_str(&l[12..]).unwrap()).collect_tuple().unwrap();
    let mut cpu = CPU {
        program_counter: 0,
        a,
        b,
        c,
    };
    let program = program[9..].split(',').map(|c| u8::from_str(c).unwrap()).collect_vec();

    let mut result = Vec::new();
    while cpu.program_counter < program.len() as u64 {
        if let Some(output) = Opcode::exec(&mut cpu, &program) {
            result.push(output);
        }
    }

    result.iter().map(|o| o.to_string()).join(",")
}

pub fn exec_day17_part2(input: &str) -> String {
    // let (registers, program) = input.trim().split("\n\n").collect_tuple().unwrap();
    // let (_, b, c) = registers.lines().map(|l| u64::from_str(&l[12..]).unwrap()).collect_tuple().unwrap();
    //
    // let program = program[9..].split(',').map(|c| u8::from_str(c).unwrap()).collect_vec();
    // let mut cache = HashMap::new();
    //
    // let mut result = 0;
    // for value_a in 0..1000000000000 {
    //     let mut cpu = CPU {
    //         program_counter: 0,
    //         a: value_a,
    //         b,
    //         c,
    //     };
    //     let mut current_output = 0;
    //     // not just cache the next output but all following outputs
    //     let mut old_cpu = cpu;
    //     while cpu.program_counter < program.len() as u64 && current_output < program.len() {
    //         if let Some((new_cpu, output)) = cache.get(&cpu) {
    //             if *output != program[current_output] as u64 {
    //                 break;
    //             }
    //             cpu = *new_cpu;
    //             current_output += 1;
    //             continue;
    //         }
    //         if let Some(output) = Opcode::exec(&mut cpu, &program) {
    //             if output != program[current_output] as u64 {
    //                 break;
    //             }
    //             cache.insert(old_cpu, (cpu, output));
    //             old_cpu = cpu;
    //             current_output += 1;
    //         }
    //     }
    //     if current_output == program.len() {
    //         result = value_a;
    //         break;
    //     }
    // }
    //
    // result.to_string()
    "".to_string()
}

#[test]
fn test_day17_part1() {
    let input = match fs::read_to_string("./example/day17.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day17_part1(&input), "4,6,3,5,6,3,5,2,1,0")
}

#[test]
fn test_day17_part2() {
    let input = match fs::read_to_string("./example/day17.txt".to_string()) {
        Ok(s) => s,
        Err(_) => panic!(),
    };
    assert_eq!(exec_day17_part2(&input), "117440")
}
