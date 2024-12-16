use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;

pub fn day10(input: String) {
    let map: Vec<Vec<u64>> = process_input(input);
    let trailheads: Vec<(usize, usize)> = find_trailheads(&map);
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for trailhead in &trailheads {
        part1 += find_peaks(*trailhead, &map).len() as u64;
    }
    for trailhead   in &trailheads {
        part2 += count_trails(*trailhead, &map);
    }
    println!("Part 1 Solution: {part1}");
    println!("Part 2 Solution: {part2}");
}

fn count_trails(point: (usize, usize), map: &Vec<Vec<u64>>) -> u64 {
    if map[point.1][point.0] == 9 { return 1 }
    let mut count: u64 = 0;
    let cur: u64 = map[point.1][point.0];
    if point.0 > 0 {
        let test_pt: u64 = map[point.1][point.0 - 1];
        if cur + 1 == test_pt { 
            count += count_trails((point.0 - 1, point.1), map); 
        }
    }
    if point.0 < map[0].len() - 1 {
        let test_pt: u64 = map[point.1][point.0 + 1];
        if cur + 1 == test_pt { 
            count += count_trails((point.0 + 1, point.1), map);
        }
    }
    if point.1 > 0 {
        let test_pt: u64 = map[point.1 - 1][point.0];
        if cur + 1 == test_pt { 
            count += count_trails((point.0, point.1 - 1), map);
        }
    }
    if point.1 < map.len() - 1 {
        let test_pt: u64 = map[point.1 + 1][point.0];
        if cur + 1 == test_pt { count += count_trails((point.0, point.1 + 1), map); }
    }
    count
}

fn find_peaks(point: (usize, usize), map: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    if map[point.1][point.0] == 9 { return vec![point] }
    let mut peaks_found: Vec<(usize, usize)> = Vec::new();
    let cur: u64 = map[point.1][point.0];
    if point.0 > 0 {
        let test_pt: u64 = map[point.1][point.0 - 1];
        if cur + 1 == test_pt { 
            peaks_found.append(&mut find_peaks((point.0 - 1, point.1), map)); 
        }
    }
    if point.0 < map[0].len() - 1 {
        let test_pt: u64 = map[point.1][point.0 + 1];
        if cur + 1 == test_pt { 
            peaks_found.append(&mut find_peaks((point.0 + 1, point.1), map)); 
        }
    }
    if point.1 > 0 {
        let test_pt: u64 = map[point.1 - 1][point.0];
        if cur + 1 == test_pt { 
            peaks_found.append(&mut find_peaks((point.0, point.1 - 1), map)); 
        }
    }
    if point.1 < map.len() - 1 {
        let test_pt: u64 = map[point.1 + 1][point.0];
        if cur + 1 == test_pt { peaks_found.append(&mut find_peaks((point.0, point.1 + 1), map)); }
    }
    peaks_found.iter().unique().map(|x| *x).collect_vec()
}

fn find_trailheads(map: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                trailheads.push((x, y))
            }
        }
    }
    trailheads
}

pub fn process_input(input: String) -> Vec<Vec<u64>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();
    let mut map: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        map.push(line.chars().map(|x| x.to_string().parse::<u64>().expect("invalid input")).collect_vec())
    }
    map
}