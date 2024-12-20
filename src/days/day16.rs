use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day16(input: String) {
    let start_map: Vec<Vec<char>> = parse_input(input);

    let mut start: (usize, usize) = (0,0);
    let mut goal: (usize, usize) = (0,0);

    for y in 0..start_map.len() {
        for x in 0..start_map[0].len() {
            if start_map[y][x] == 'S' { start = (x, y) }
            else if start_map[y][x] == 'E' { goal = (x, y) }
        }
    }
    
    println!("Part 1 Solution: {}", lowest_score(&start, &goal, &mut start_map.clone(), '>'));
}

// recursive pathfinding function
fn lowest_score(pos: &(usize, usize), goal: &(usize, usize), test_map: &mut Vec<Vec<char>>, dir: char) -> usize {
    if pos.eq(goal) { return 0 }

    test_map[pos.1][pos.0] = 'X';

    _print_map(test_map);

    let mut test_map_2: Vec<Vec<char>> = test_map.clone();

    let mut scores: Vec<usize> = vec![usize::MAX];

    // test going up
    if get_point(&test_map, (pos.0 as i32, pos.1 as i32 - 1)) == '.' {
        scores.push(safe_add(get_motion_cost(dir, '^'), lowest_score(&(pos.0, pos.1 - 1), goal, &mut test_map_2, '^')))
    }

    // test going left
    if get_point(&test_map, (pos.0 as i32 - 1, pos.1 as i32)) == '.' {
        scores.push(safe_add(get_motion_cost(dir, '<'), lowest_score(&(pos.0 - 1, pos.1), goal, &mut test_map_2, '<')))
    }

    // test going down
    if get_point(&test_map, (pos.0 as i32, pos.1 as i32 + 1)) == '.' {
        scores.push(safe_add(get_motion_cost(dir, 'v'), lowest_score(&(pos.0, pos.1 + 1), goal, &mut test_map_2, 'v')))
    }

    // test going right
    if get_point(&test_map, (pos.0 as i32 + 1, pos.1 as i32)) == '.' {
        scores.push(safe_add(get_motion_cost(dir, '>'), lowest_score(&(pos.0 + 1, pos.1), goal, &mut test_map_2, '>')))
    }

    let score: usize = *scores.iter().min().unwrap();
    if score == usize::MAX {
        test_map[pos.1][pos.0] = '#';
    }
    score
}

// debug function
fn _print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for entry in line {
            print!("{entry}")
        }
        print!{"\n"};
    }
}

// add without overflowing
fn safe_add(a:usize, b:usize) -> usize {
    if usize::MAX - a < b || usize::MAX - b < a { usize::MAX }
    else { a + b }
}

fn get_motion_cost(a: char, b: char) -> usize {
    if a == b || b == '*' { 1 }
    else if a == '^' && b == 'v' || a == 'v' && b == '^' || a == '<' && b == '>' || a == '>' && b == '<' { 2001 }
    else { 1001 }
}

// safely get point or bogus value if out of bounds
fn get_point(map: &Vec<Vec<char>>, pt: (i32, i32)) -> char {
    if pt.0 < 0 || pt.0 >= map[0].len() as i32 || pt.1 < 0 || pt.1 >= map.len() as i32 { '#' }
    else {
        let res =  map[pt.1 as usize][pt.0 as usize];
        if res == 'E' { '.' } else { res }
    }
}


fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}