use std::{cmp::min, fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day16(input: String) {
    let mut in_map: Vec<Vec<char>> = parse_input(input);
    let mut pos: (usize, usize) = (0,0);

    for y in 0..in_map.len() {
        for x in 0..in_map[0].len() {
            if in_map[y][x] == 'S' {
                pos = (x, y);
                in_map[y][x] = '>';
            }
        }
    }

    println!("Part 1 Solution: {}", min_path(&in_map, pos, '>'));
}

pub fn min_path(map: &Vec<Vec<char>>, pos: (usize, usize), orig: char) -> usize {
    let this: char = map[pos.1][pos.0];
    let next_step: (usize, usize);
    let next_move: char;
    let _prev_move: char;

    match this {
        '^' => {
            next_step = (pos.0, pos.1 - 1);
            next_move = '<';
            _prev_move = '>';
        }
        '>' => {
            next_step = (pos.0 + 1, pos.1);
            next_move = '^';
        }
        'v' => {
            next_step = (pos.0, pos.1 + 1);
            next_move = '>';
        }
        '<' => {
            next_step = (pos.0 - 1, pos.1);
            next_move = 'v';
        }
        _ => return usize::MAX
    }

    let next_val = map[next_step.1][next_step.0];
    if next_val == 'E' { return 1 }
    else {
        let mut turn_path: usize = usize::MAX;

        if next_move != orig {
            let mut map_turn: Vec<Vec<char>> = map.clone();
            map_turn[pos.1][pos.0] = next_move;
            turn_path = min_path(&map_turn, pos, orig);
            if turn_path != usize::MAX { turn_path += 1000 }
            drop(map_turn);
        }

        if next_val == '.' {
            let mut map_straight: Vec<Vec<char>> = map.clone();
            map_straight[next_step.1][next_step.0] = this;
            let mut straight_path:usize = min_path(&map_straight, next_step, this);
            if straight_path != usize::MAX { straight_path += 1 }
            drop(map_straight);

            return min(turn_path, straight_path)
        } else { return turn_path; }
    }
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}