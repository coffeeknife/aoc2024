use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref XMAS: Regex = Regex::new(r"XMAS|SAMX").unwrap();
    static ref MAS: Regex = Regex::new(r"MAS|SAM").unwrap();
}

pub fn day4(input: String) {
    let wordsearch = parse_input(input);
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for y in 0..wordsearch.len() {
        let row = wordsearch.get(y).unwrap();
        for x in 0..row.len() {
            // PART 2 LOGIC
            if x < row.len() - 2 && y < wordsearch.len() - 2 {
                let row2 = wordsearch.get(y+1).unwrap();
                let row3 = wordsearch.get(y+2).unwrap();
                let diag = format!("{}{}{}",
                    row.get(x).unwrap(),
                    row2.get(x+1).unwrap(),
                    row3.get(x+2).unwrap()
                );
                let rdiag = format!("{}{}{}",
                    row.get(x+2).unwrap(),
                    row2.get(x+1).unwrap(),
                    row3.get(x).unwrap()
                );
                if MAS.is_match(diag.as_str()) && MAS.is_match(rdiag.as_str()) {
                    part2 += 1;
                }
            }
            // PART 1 LOGIC 
            if x < row.len() - 3 {
                let horiz = format!("{}{}{}{}", 
                    row.get(x).unwrap(),
                    row.get(x+1).unwrap(),
                    row.get(x+2).unwrap(),
                    row.get(x+3).unwrap()
                );
                if XMAS.is_match(horiz.as_str()) { 
                    part1 += 1;
                }
            }
            if y < wordsearch.len() - 3 {
                let row2 = wordsearch.get(y+1).unwrap();
                let row3 = wordsearch.get(y+2).unwrap();
                let row4 = wordsearch.get(y+3).unwrap();
                let vert = format!("{}{}{}{}",
                    row.get(x).unwrap(),
                    row2.get(x).unwrap(),
                    row3.get(x).unwrap(),
                    row4.get(x).unwrap()
                );
                if XMAS.is_match(vert.as_str()) { 
                    part1 += 1;
                }
                if x < row.len() - 3 {
                    let diag = format!("{}{}{}{}",
                        row.get(x).unwrap(),
                        row2.get(x+1).unwrap(),
                        row3.get(x+2).unwrap(),
                        row4.get(x+3).unwrap()
                    );
                    let rdiag = format!("{}{}{}{}",
                        row.get(x+3).unwrap(),
                        row2.get(x+2).unwrap(),
                        row3.get(x+1).unwrap(),
                        row4.get(x).unwrap()
                    );
                    if XMAS.is_match(diag.as_str()) { 
                        part1 += 1;
                    }
                    if XMAS.is_match(rdiag.as_str()) { 
                        part1 += 1; 
                    }
                }
            }
        }
    }
    println!("Part 1 Solution: {part1}");
    println!("Part 2 Solution: {part2}");
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}