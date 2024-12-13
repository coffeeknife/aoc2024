use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day5(input: String) {
    println!("[RUNNING DAY 5]");
    let (order_rules, updates) = parse_input(input);
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for update in updates {
        let mut occured_before: Vec<u64> = Vec::new();
        let mut safe: bool = true;
        for page in &update {
            if order_rules.contains_key(page) {
                for req in order_rules.get(page).unwrap() {
                    if !occured_before.contains(req) && update.contains(req) {
                        safe = false;
                        break;
                    }
                }
            }
            if !safe { break; }
            occured_before.push(*page);
        }
        if safe {
            let i = (update.len() - 1) / 2;
            part1  += update.get(i).unwrap();
        } else {
            let fixed:Vec<u64> = fix_update(&order_rules, &update);
            part2 += fixed.get((fixed.len() - 1) / 2).unwrap();
        }
    }
    println!("Part 1 Solution: {part1}");
    println!("Part 2 Solution: {part2}")
}

fn fix_update(order_rules: &HashMap<u64, Vec<u64>>, update: &Vec<u64>) -> Vec<u64> {
    let mut fixed_update: Vec<u64> = Vec::new();
    for page in update {
        if fixed_update.contains(&page) { continue; } //this means we reordered it
        let mut needs_fixing = false;
        if order_rules.contains_key(&page) {
            for req in order_rules.get(&page).unwrap() {
                if update.contains(&req) && !fixed_update.contains(&req) {
                    fixed_update.push(*req);
                    needs_fixing = true;
                }
            }
        }  
        if needs_fixing { fixed_update = fix_update(order_rules, &fixed_update) }
        fixed_update.push(*page);
    }
    fixed_update
}

pub fn parse_input(input: String) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();

    let mut order_rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut updates: Vec<Vec<u64>> = Vec::new();
    let mut rules_done: bool = false;

    for line in lines {
        if line.is_empty() { rules_done = true; continue; }
        if !rules_done {
            let mut parsed = line.split("|").map(|x| x.parse::<u64>().unwrap());
            let (a, b) = (parsed.next().unwrap(), parsed.next().unwrap());
            order_rules.entry(b).or_insert_with(Vec::new).push(a)
        } else {
            updates.push(line.split(",").map(|x| x.parse::<u64>().unwrap()).collect())
        }
    }

    (order_rules, updates)
}