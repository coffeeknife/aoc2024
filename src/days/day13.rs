use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone)]
struct ClawMachine {
    pub move_a: (u128, u128),
    pub move_b: (u128, u128),
    pub prize: (u128, u128)
}

static ERR_CORRECT: u128 = 10000000000000;

lazy_static!(
    static ref BTN: Regex = Regex::new(r"Button [A|B]: X\+(\d+)\, Y\+(\d+)").unwrap();
    static ref PRIZE: Regex = Regex::new(r"Prize: X\=(\d+), Y\=(\d+)").unwrap();
);

pub fn day13(input: String) {
    let machines: Vec<ClawMachine> = parse_input(input);
    println!("Part 1 Solution: {}", total_prize_cost(&machines));
    
    let machines_corrected: Vec<ClawMachine> = machines.iter().map(|x| ClawMachine{
        move_a: x.move_a,
        move_b: x.move_b,
        prize: (x.prize.0 + ERR_CORRECT, x.prize.1 + ERR_CORRECT),
    }).collect_vec();

    println!("Part 2 Solution: {}", total_prize_cost(&machines_corrected));
}

fn total_prize_cost(machines: &Vec<ClawMachine>) -> u128 {
    let mut cost: u128 = 0;
    for machine in machines {
        // use big math data types to not screw up the value
        let divisor: f64 = (machine.move_a.0 * machine.move_b.1) as f64 - (machine.move_b.0 * machine.move_a.1) as f64;
        let a: f64 = ((machine.move_b.1 * machine.prize.0) as f64 - (machine.move_b.0 * machine.prize.1) as f64) / divisor;
        let b: f64 = ((machine.move_a.0 * machine.prize.1) as f64 - (machine.move_a.1 * machine.prize.0) as f64) / divisor;
        if a == (a as u128) as f64 && b == (b as u128) as f64 {
            cost += (a as u128 * 3) + b as u128;
        }
    }
    cost
}

fn parse_input(input: String) -> Vec<ClawMachine> {
    let mut machines: Vec<ClawMachine> = Vec::new();

    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten().collect_vec();

    let mut i:usize = 0;
    while i < lines.len() - 2 {
        let (line1, line2, line3) = (lines[i].clone(), lines[i+1].clone(), lines[i+2].clone());

        let move_a: [u128; 2]  = BTN.captures(&line1).unwrap().extract::<2>().1.map(|x| x.parse::<u128>().unwrap());
        let move_b: [u128; 2]  = BTN.captures(&line2).unwrap().extract::<2>().1.map(|x| x.parse::<u128>().unwrap());
        let prize: [u128; 2] = PRIZE.captures(&line3).unwrap().extract::<2>().1.map(|x| x.parse::<u128>().unwrap());

        machines.push(ClawMachine{
            move_a: (move_a[0], move_a[1]),
            move_b: (move_b[0], move_b[1]),
            prize: (prize[0], prize[1])
        });
        i += 4;
    }

    machines
}