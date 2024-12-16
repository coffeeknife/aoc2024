use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day8(input: String) {  // TODO rewrite by just recording inside the string like day 6
    println!("[RUNNING DAY 8]");
    let (dimensions, antennae) = parse_input(input);
    let mut antinodes: Vec<(usize, usize)> = Vec::new();
    for key in antennae.keys() {
        let mut points: Vec<(usize, usize)> = antennae.get(key).unwrap().clone();
        while points.len() > 1 {
            let cur: (usize, usize) = points.pop().expect("Somehow there are no points?");
            for comp in &points {
                let (anti1, anti2) = get_antinodes(cur, *comp);
                if anti1.0 >= 0 && (anti1.0 as usize) < dimensions.0 && anti1.1 >= 0 && (anti1.1 as usize) < dimensions.1 {
                    let a = (anti1.0 as usize, anti1.1 as usize);
                    if !check_contains(&antinodes, &a) { antinodes.push(a); }
                }
                if anti2.0 >= 0 && (anti2.1 as usize) < dimensions.0 && anti2.1 >= 0 && (anti2.1 as usize) < dimensions.1 {
                    let a = (anti2.0 as usize, anti2.1 as usize);
                    if !check_contains(&antinodes, &a) { antinodes.push(a); }
                }
            }
        }
    }
    println!("Part 1 Solution: {}", antinodes.len())
}

fn check_contains(antinodes: &Vec<(usize, usize)>, node: &(usize, usize)) -> bool {
    for test in antinodes {
        if test.0 == node.0 && test.1 == node.1 {
            return true;
        }
    }
    false
}

fn get_antinodes(point1: (usize, usize), point2: (usize, usize)) -> ((i64, i64), (i64, i64)) {
    let diff_x: i64 = if point1.0 > point2.0 { point1.0 - point2.0 } else { point2.0 - point1.0 } as i64;
    let diff_y: i64 = if point1.1 > point2.1 { point1.1 - point2.1 } else { point2.1 - point1.1 } as i64;

    let node_1 = (
        if point1.0 > point2.0 { point1.0 as i64 + diff_x } else { point1.0 as i64 - diff_x },
        if point1.1 > point2.1 { point1.1 as i64 + diff_y } else { point1.1 as i64 - diff_y }
    );
    let node_2 = (
        if point1.0 > point2.0 { point2.0 as i64 - diff_x } else { point2.0 as i64 + diff_x },
        if point1.1 > point2.1  { point2.1 as i64 - diff_y } else { point2.1 as i64 + diff_y }
    );

    (node_1, node_2)
}

fn parse_input(input: String) -> ((usize, usize), HashMap<char, Vec<(usize, usize)>>) {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let char_grid: Vec<Vec<char>> = BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect();

    let dimensions: (usize, usize) = (char_grid[0].len(), char_grid.len());
    let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let point = char_grid[y][x];
            if point != '.' {
                antennae.entry(point).or_insert(Vec::new()).push((x, y));
            }
        }
    }

    (dimensions, antennae)
}