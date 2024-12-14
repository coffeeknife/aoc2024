use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day8(input: String) {
    println!("[RUNNING DAY 8]");
    let (dimensions, antennae) = parse_input(input);
    let mut antinodes: Vec<(usize, usize)> = Vec::new();
    for key in antennae.keys() {
        let mut points: Vec<(usize, usize)> = antennae.get(key).unwrap().clone();
        while points.len() > 1 {
            let cur: (usize, usize) = points.pop().expect("Somehow there are no points?");
            for comp in &points {
                let dif_x: usize = ((cur.0 as i64) - (comp.0 as i64)).abs() as usize;
                let dif_y: usize = ((cur.1 as i64) - (comp.1 as i64)).abs() as usize;

                let mut anti1: (usize, usize) = (0, 0);
                let mut anti2: (usize, usize) = (0, 0);

                let mut anti1_good: bool = true;
                let mut anti2_good: bool = true;

                if cur.0 > comp.0 {
                    if cur.0 + dif_x >= dimensions.0 { anti1_good = false; }
                    else { anti1.0 = cur.0 + dif_x; }
                    if comp.0 < dif_x { anti2_good = false; }
                    else { anti2.0 = comp.0 - dif_x; }
                } else {
                    if cur.0 < dif_x { anti1_good = false; }
                    else { anti1.0 = cur.0 - dif_x; }
                    if comp.0 + dif_x >= dimensions.0 { anti2_good = false; }
                    else { anti2.0 = comp.0 - dif_x; }
                }

                if cur.1 > comp.1 {
                    if cur.1 + dif_y >= dimensions.1 { anti1_good = false; }
                    else { anti1.1 = cur.1 + dif_y; }
                    if comp.1 < dif_y { anti2_good = false; }
                    else { anti2.1 = comp.1 - dif_y; }
                } else {
                    if cur.1 < dif_y { anti1_good = false; }
                    else { anti1.1 = cur.1 - dif_y; }
                    if comp.1 + dif_y >= dimensions.1 { anti2_good = false; }
                    else { anti2.1 = comp.1 + dif_y; }
                }
                if anti1_good && !antinodes.contains(&anti1) { antinodes.push(anti1) }
                if anti2_good && !antinodes.contains(&anti2) { antinodes.push(anti2) }               
            }
        }
        println!("After {key}: {:?}", antinodes);
    }
    println!("Part 1 Solution: {}", antinodes.len())
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