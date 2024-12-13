use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FULL: Regex = Regex::new(r"mul\(\d+\,\d+\)|do\(\)|don\'t\(\)").unwrap();
    static ref MUL: Regex = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    static ref DO: Regex = Regex::new(r"do\(\)").unwrap();
    static ref DONT: Regex = Regex::new(r"don\'t\(\)").unwrap();
}

pub fn day3(input: String) {
    let data = parse_input(input);
    let matches: Vec<&str> = FULL.find_iter(&data).map(|s| s.as_str()).collect();
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    let mut enabled = true;
    for mul in matches {
        if DO.is_match(mul) {
            enabled = true;
        } else if DONT.is_match(mul) {
            enabled = false;
        } else if MUL.is_match(mul) {
            let mut parsed = mul[4..(mul.len() - 1)].split(",").map(|x| x.parse::<u64>().unwrap());
            let (a, b) = (parsed.next().expect("Parse error"), parsed.next().expect("Parse error"));
            let res = a * b;
            part1 += res;
            if enabled { part2 += res }
        }
    }
    println!("Part 1 Solution: {part1}");
    println!("Part 2 Solution: {part2}");
}

fn parse_input(input: String) -> String {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().join("")
}