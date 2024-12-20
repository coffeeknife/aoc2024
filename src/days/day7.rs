use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;

pub fn day7(input: String) {
    let data: HashMap<u64, Vec<u64>> = parse_input(input);
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for key in data.keys() {
        let results = get_all_combos(data.get(key).unwrap(), false);
        let results_concat = get_all_combos(data.get(key).unwrap(), true);
        if results.contains(key) {
            part1 += key;
        }
        if results_concat.contains(key) {
            part2 += key;
        }
    }
    println!("Part 1 Result: {part1}");
    println!("Part 2 Result: {part2}");
}

fn get_all_combos(input: &Vec<u64>, concat: bool) -> Vec<u64> {
    let mut input_clone = input.clone();
    let mut results: Vec<u64> = Vec::new();
    if input.len() != 0 { 
        let last = input_clone.pop().unwrap();
        let subresults = get_all_combos(&input_clone, concat);
        if subresults.is_empty() { results.push(last) }
        else {
            for subresult in subresults {
                results.push(subresult + last);
                results.push(subresult * last);
                if concat {
                    results.push(format!("{}{}",subresult,last).parse::<u64>().unwrap())
                }
            }

        }
    }
    results
}

fn parse_input(input: String) -> HashMap<u64, Vec<u64>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();

    let mut data: HashMap<u64, Vec<u64>> = HashMap::new();

    for line in lines {
        let parse = line.split(":").collect_vec();
        let result = parse[0].parse::<u64>().unwrap();
        if data.contains_key(&result) { println!("WARNING: duplicate results found in data! rewrite required.") }
        let vals = parse[1].trim().split(' ').map(|x| x.parse::<u64>().unwrap()).collect_vec();
        data.insert(result, vals);
    }

    data
}