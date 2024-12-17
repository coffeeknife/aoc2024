use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;

pub fn day15(input: String) {
    let (map, moves) = process_input(input);
    let map_widened = map.iter().map(|x| {
        let mut new: Vec<char> = Vec::new();
        for chr in x {
            match chr {
                '#' => { new.push('#'); new.push('#') }
                '.' => { new.push('.'); new.push('.') }
                'O' => { new.push('['); new.push(']') }
                '@' => { new.push('@'); new.push('.') }
                _ => ()
            }
        }
        new
    }).collect_vec();

    let map_pt1: Vec<Vec<char>> = robot_move(map, moves.clone());
    let map_pt2: Vec<Vec<char>> = robot_move(map_widened.clone(), moves.clone()); 

    let mut pt1: usize = 0;

    for y in 0..map_pt1.len() {
        for x in 0..map_pt1[0].len() {
            if map_pt1[y][x] == 'O' {
                pt1 += (100 * y) + x;
            }
        }
    }

    let mut pt2: usize = 0;

    for y in 0..map_pt2.len() {
        for x in 0..map_pt2[0].len() {
            if map_pt2[y][x] == '[' {
                pt2 += (100 * y) + x;
            }
        }
    }

    for line in map_pt2 {
        println!("{}", line.iter().join(""))
    }

    println!("Part 1 Solution: {pt1}");
    println!("Part 2 Solution: {pt2}");
}

fn robot_move(map_start: Vec<Vec<char>>, moves: Vec<char>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = map_start.clone();
    let mut pos: (i32, i32) = find_bot(&map);

    for line in &map {
        println!("{}", line.iter().join(""))
    }

    for mv in moves {
        pos = step(&mut map, pos, mv);
        for line in &map {
            println!("{}", line.iter().join(""))
        }
    }

    map
}

fn step (map: &mut Vec<Vec<char>>, pos: (i32, i32), stepv: char) -> (i32, i32) {
    let mut next_pos: (i32, i32) = (0, 0);
    let this: char = get_point(map, pos);

    match stepv {
        '^' => next_pos = (pos.0, pos.1 - 1),
        '>' => next_pos = (pos.0 + 1, pos.1),
        'v' => next_pos = (pos.0, pos.1 + 1),
        '<' => next_pos = (pos.0 - 1, pos.1),
        _ => println!("Warning: unrecognized step {stepv}")
    }

    match get_point(map, next_pos) {
        '.' => {
            if this == ']' {
                let adj_pos: (i32, i32) = (pos.0 - 1, pos.1);
                if get_point(map, adj_pos) == '[' {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    if !step(map, adj_pos, stepv).eq(&adj_pos) {
                        map[next_pos.1 as usize][next_pos.0 as usize] = this;
                        return next_pos;
                    } else {
                        map[pos.1 as usize][pos.0 as usize] = this;
                        return pos;
                    }
                }
            } else if this == '[' {
                let adj_pos: (i32, i32) = (pos.0 + 1, pos.1);
                if get_point(map, adj_pos) == ']' {
                    map[pos.1 as usize][pos.0 as usize] = '.';
                    if !step(map, adj_pos, stepv).eq(&adj_pos) {
                        map[next_pos.1 as usize][next_pos.0 as usize] = this;
                        return next_pos;
                    } else {
                        map[pos.1 as usize][pos.0 as usize] = this;
                        return pos;
                    }
                }
            }
            map[next_pos.1 as usize][next_pos.0 as usize] = this;
            map[pos.1 as usize][pos.0 as usize] = '.';
            return next_pos;
        }
        'O' | ']' | '[' => {
            if !step(map, next_pos, stepv).eq(&next_pos) {
                return step(map, pos, stepv); // only recursive call if there is motion
            }
        }
        _ => {} // ignore anything else
    }

    pos
}

// safely get point or bogus value if out of bounds
fn get_point(map: &Vec<Vec<char>>, pt: (i32, i32)) -> char {
    if pt.0 < 0 || pt.0 >= map[0].len() as i32 || pt.1 < 0 || pt.1 >= map.len() as i32 { '#' }
    else { map[pt.1 as usize][pt.0 as usize] }
}

fn find_bot(map: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                return (x as i32, y as i32);
            }
        }
    }
    (i32::MAX, i32::MAX)
}

fn process_input(input: String) -> (Vec<Vec<char>>, Vec<char>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = Vec::new();

    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();

    let mut map_done = false;
    for line in lines {
        if line == "" { map_done = true; continue; }
        if !map_done {
            map.push(line.chars().collect_vec());
        } else {
            moves.append(&mut line.chars().collect_vec());
        }
    }
    (map, moves)
}