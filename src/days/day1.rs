use std::{fs::File, io::{self, BufRead}, path::Path};

pub fn day1(input: String) {
    let (left, right) = parse_input(input);
    let mut part1: u64 = 0;
    for i in 0..left.len() {
        part1 += ((*left.get(i).unwrap() as i64) - (*right.get(i).unwrap() as i64)).abs() as u64;
    }
    println!("Part 1 Solution: {part1}");
}

fn parse_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = io::BufReader::new(file).lines().flatten();

    for line in lines {
        let mut parts = line.split_whitespace().map(|s| s.parse::<u64>());
        match (parts.next(), parts.next()) {
            (Some(Ok(a)), Some(Ok(b))) => {
                left.push(a);
                right.push(b);
            }
            _ => {
                println!("Encountered an error reading line {line}");
            }
        }
    }
    left.sort();
    right.sort();
    (left, right)
}