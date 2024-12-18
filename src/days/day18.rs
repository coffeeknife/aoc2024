use std::{cmp::max, fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, set_progress_bar_progress, Color, Style};

static SIZE: usize = 71; // set in test vs prod
static SIMULATE: usize = 1024;
static MAX: usize = SIZE * SIZE + 1; // no valid path can be this long 

pub fn day18(input: String) {
    let coords: Vec<(usize, usize)> = parse_input(input);
    let mut mem_map: Vec<Vec<usize>> = vec![vec![MAX; SIZE]; SIZE];

    // initialise dist map
    for y in 0..SIZE {
        for x in 0..SIZE {
            if x == 0 && y == 0 { mem_map[y][x] = 0 }
            else { mem_map[y][x] = 1 + get_min_val(&mem_map, get_adjacent((x, y))) }
        }
    }

    let mut part1: usize = 0;
    let mut part2: (usize, usize) = (0, 0);

    init_progress_bar(coords.len());
    set_progress_bar_action("Simulating...", Color::Blue, Style::Bold);
    for i in 0..coords.len() {
        let coord: (usize, usize) = *coords.get(i).unwrap();
        mem_map[coord.1][coord.0] = MAX;
        let adjacent: Vec<(usize, usize)> = get_adjacent(coord);
        for pt in adjacent {
            cascade_update(&mut mem_map, pt);
        }

        if mem_map[SIZE - 1][SIZE - 1] >= MAX {
            set_progress_bar_progress(coords.len());
            finalize_progress_bar();
            part2 = coord;
            break;
        }
        if i == SIMULATE-1 { part1 = mem_map[SIZE - 1][SIZE - 1]; }
        inc_progress_bar();
    }

    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}, {}", part2.0, part2.1);
}

fn cascade_update(map: &mut Vec<Vec<usize>>, pt: (usize, usize)) {
    let adj_val: usize = map[pt.1][pt.0];
    let mut new_min: usize = get_min_val(&map, get_adjacent(pt));
    if new_min != MAX { new_min += 1; }
    new_min = max(new_min, adj_val);
    map[pt.1][pt.0] = new_min;

    if new_min != adj_val {
        let new_adj: Vec<(usize, usize)> = get_adjacent(pt);
        for p in new_adj { cascade_update(map, p); }
    }
}

// debug function
fn _print_map(map: &Vec<Vec<usize>>) {
    for line in map {
        for entry in line {
            if *entry >= MAX { print!("## ") }
            else { print!("{:02} ", entry) }
        }
        print!{"\n"};
    }
}

fn get_min_val(map: &Vec<Vec<usize>>, points: Vec<(usize, usize)>) -> usize {
    let mut vals: Vec<usize> = Vec::new();
    for point in points { vals.push(map[point.1][point.0]) }
    *vals.iter().min().unwrap()
}

// get adjacent points ordered by distance from 0,0
fn get_adjacent(pt: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj: Vec<(usize, usize)> = Vec::new();
    if pt.0 > 0 { adj.push((pt.0 - 1, pt.1)) }
    if pt.1 > 0 { adj.push((pt.0, pt.1 - 1)) }
    if pt.0 < SIZE - 1 { adj.push((pt.0 + 1, pt.1)) }
    if pt.1 < SIZE - 1 { adj.push((pt.0, pt.1 + 1)) }
    adj
}

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x| x.split(',').map(|y| y.parse::<usize>().unwrap()).collect_tuple().unwrap()).collect_vec()
}