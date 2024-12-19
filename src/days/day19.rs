use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color, Style};
use substring::Substring;

pub fn day19(input: String) {
    let (designs, patterns) = parse_input(input);
    let mut cache: HashMap<String, usize> = HashMap::new();

    let mut possible_designs: usize = 0;
    let mut design_count: usize = 0;
    
    init_progress_bar(designs.len());
    set_progress_bar_action("Solving...", Color::Blue, Style::Bold);
    for design in designs {
        let count = num_possible(&design, &patterns, &mut cache);
        design_count += count;
        if count != 0 { possible_designs += 1 }
        inc_progress_bar();
    }
    finalize_progress_bar();

    println!("Part 1 Solution: {possible_designs}");
    println!("Part 2 Solution: {design_count}");
}

fn num_possible(design: &String, patterns: &Vec<String>, cache: &mut HashMap<String, usize>) -> usize {

    if design.len() == 0 { return 1; }
    else if cache.keys().contains(design) {
        return *cache.get(design).unwrap();
    }
    let mut possible: usize = 0;

    for pattern in patterns.iter().filter(|x: &&String| (*x).substring(0, 1).eq(design.substring(0,1))) {
        if design.eq(pattern) {
            possible += 1
        } else if design.len() > pattern.len() && design.substring(0, pattern.len()).eq(pattern) {
            let substring = design.substring(pattern.len(), design.len()).to_string();
            if cache.keys().contains(&substring) {
                possible += cache.get(&substring).unwrap();
            } else {
                let subpossible = num_possible(&substring, patterns, cache);
                possible += subpossible;
            }
        }
    }

    cache.entry(design.clone()).and_modify(|x| *x += possible).or_insert(possible);
    possible
}

fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();

    let mut got_patterns: bool = false;
    let mut designs: Vec<String> = Vec::new();
    let mut patterns: Vec<String> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if !got_patterns { patterns.append(&mut line.clone().split(", ").map(|x| x.to_string()).collect_vec()); got_patterns = true; }
        else { designs.push(line.clone()) }
    }
    (designs, patterns)
}