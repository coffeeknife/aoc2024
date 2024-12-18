use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use lazy_static::lazy_static;
use num::pow;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, set_progress_bar_progress, Color, Style};
use regex::Regex;

lazy_static!(
    static ref REG: Regex = Regex::new(r"Register [A|B|C]: (\d+)").unwrap();
    static ref PROG: Regex = Regex::new(r"Program: ([\d|\,]+)").unwrap();
);

pub fn day17(input: String) {
    let reg_a: usize;
    let reg_b: usize;
    let reg_c: usize;
    let program: Vec<usize>;
    (reg_a, reg_b, reg_c, program) = parse_input(input);

    let output_pt1: Vec<usize> = run(&program, (reg_a, reg_b, reg_c));
    println!("Part 1 Solution: {}", output_pt1.iter().join(","));

    // the program handles 3 bits of A at a time until there are none left, so we can narrow our search range
    let min: usize = 2usize.pow((program.len() as u32 * 3) - 2);
    let max: usize = 2usize.pow((program.len() as u32 * 3) + 1);

    init_progress_bar(max - min);
    set_progress_bar_action("Solving Pt2", Color::Blue, Style::Bold);
    for i in min..max {
        let test_output: Vec<usize> = run(&program, (i, 0, 0));
        if test_output.eq(&program) {
            set_progress_bar_progress(max - min);
            finalize_progress_bar();
            println!("Part 2 Solution: {i}");
            break;
        }
        inc_progress_bar();
    }
}

fn run(program: &Vec<usize>, registers: (usize, usize, usize)) -> Vec<usize> {
    let mut reg_a: usize = registers.0;
    let mut reg_b: usize = registers.1;
    let mut reg_c: usize = registers.2;
    let mut output: Vec<usize> = Vec::new();

    let mut i: usize = 0;
    while i < program.len() - 1 {

        let opcode: usize = program[i];
        let operand: usize = program[i+1];

        match opcode {
            0 => {
                reg_a = reg_a / pow(2, combo(operand, (reg_a, reg_b, reg_c)));
            }
            1 => {
                reg_b = reg_b ^ operand;
            }
            2 => {
                reg_b = combo(operand, (reg_a, reg_b, reg_c)) % 8;
            }
            3 => {
                if reg_a != 0 { i = operand; continue; }
            }
            4 => {
                reg_b = reg_b ^ reg_c;
            }
            5 => {
                output.push(combo(operand, (reg_a, reg_b, reg_c)) % 8);
            }
            6 => {
                reg_b = reg_a / pow(2, combo(operand, (reg_a, reg_b, reg_c)));
            }
            7 => {
                reg_c = reg_a / pow(2, combo(operand, (reg_a, reg_b, reg_c)));
            }
            _ => println!("WARN: impossible operand {opcode}")
        }
        i += 2;
    }

    output
}

fn combo(operand: usize, registers: (usize, usize, usize)) -> usize {
    match operand {
        0..=3 => operand,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => usize::MAX
    }
}

pub fn parse_input(input: String) -> (usize, usize, usize, Vec<usize>) {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let mut lines = BufReader::new(file).lines().flatten();

    let reg_a: usize = REG.captures(&lines.next().unwrap()).unwrap().extract::<1>().1[0].parse::<usize>().unwrap();
    let reg_b: usize = REG.captures(&lines.next().unwrap()).unwrap().extract::<1>().1[0].parse::<usize>().unwrap();
    let reg_c: usize = REG.captures(&lines.next().unwrap()).unwrap().extract::<1>().1[0].parse::<usize>().unwrap();
    let _ = &lines.next().unwrap();
    let program: Vec<usize> = PROG.captures(&lines.next().unwrap()).unwrap().extract::<1>().1[0].split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
    
    (reg_a, reg_b, reg_c, program)
}