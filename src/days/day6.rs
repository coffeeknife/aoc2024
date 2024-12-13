use core::panic;
use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color, Style};

pub fn day6(input: String) {
    println!("[RUNNING DAY 6]");
    let mut map: Vec<Vec<char>> = parse_input(input);
    let map_original: Vec<Vec<char>> = map.clone();
    let (mut guard_x, mut guard_y) = find_guard(&map);
    let mut pos_count: u64 = 1;
    let mut guard_replace = '^';
    loop {
        (guard_x, guard_y, pos_count, guard_replace) = move_guard(&mut map, (guard_x, guard_y), &pos_count,guard_replace);
        if guard_x == usize::MAX || guard_y == usize::MAX {
            break;
        }
    }
    println!("Part 1 Result: {pos_count}");

    // part 2 is very slow because my code is unoptimized, so we add a progress bar
    init_progress_bar(map_original.len() * map_original[0].len());
    set_progress_bar_action("Solving Pt 2:", Color::Blue, Style::Bold);

    // now we iterate over the entire original map and see if we can cause a loop.
    let mut loop_count: u64 = 0;
    for y in 0..map_original.len() {
        for x in 0..map_original.get(0).unwrap().len() {
            inc_progress_bar();
            if map_original[y][x] != '.' { continue } // can only test unobstructed points
            let mut test_map = map_original.clone();
            test_map[y][x] = '#';
            let mut rep_guard = '^';
            let (mut guard_x, mut guard_y) = find_guard(&test_map);
            loop {
                let (new_guard_x, new_guard_y, _, new_rep) = move_guard(&mut test_map, (guard_x, guard_y), &0, rep_guard);
                rep_guard = new_rep;
                if new_guard_x == usize::MAX || new_guard_y == usize::MAX { 
                    break 
                } // guard left map; no loop
                else if new_guard_x == guard_x && new_guard_y == guard_y {
                    loop_count += 1;
                    break
                }
                (guard_x, guard_y) = (new_guard_x, new_guard_y)
            }
        }
    }
    finalize_progress_bar();
    println!("Part 2 Result: {loop_count}");
}

fn check_pos(map: &Vec<Vec<char>>, pos: (usize, usize)) -> char {
    let (x, y) = pos.clone();
    *(map.get(y).unwrap().get(x).unwrap())
}

fn move_guard(map: &mut Vec<Vec<char>>, pos: (usize, usize), pos_count: &u64, guard_replace: char) -> (usize, usize, u64, char) {
    let upper_x = map.get(0).unwrap().len() - 1;
    let upper_y = map.len() - 1;
    let (mut guard_x, mut guard_y) = pos.clone();
    let mut updated_count = *pos_count;
    let guard = map[pos.1][pos.0];
    match guard {
        '^' => {
            if guard_y == 0 { return (usize::MAX, usize::MAX, updated_count, '*') } // avoid usize errors
            guard_y -= 1
        }
        'v' => {
            if guard_y == upper_y { return (usize::MAX, usize::MAX, updated_count, '*') }
            guard_y += 1
        },
        '>' => {
            if guard_x == upper_x { return (usize::MAX, usize::MAX, updated_count, '*') }
            guard_x += 1
        },
        '<' => {
            if guard_x == 0 { return (usize::MAX, usize::MAX, updated_count, '*') }
            guard_x -= 1
        },
        _ => panic!("Lost track of the guard somehow?")
    }
    let new_pos: char = check_pos(map, (guard_x, guard_y));
    if new_pos == '#' {
        let next: char = get_next(&guard);
        if detect_loop(next, guard) || detect_loop(next, guard_replace) { // loop
            return (pos.0, pos.1, updated_count, guard_replace);
        }
        map[pos.1][pos.0] = next;
        return move_guard(map, pos, pos_count, guard_replace);
    } else if detect_loop(guard, new_pos) {
        // loop detected
        return (pos.0, pos.1, updated_count, guard_replace); // we will indicate this by no change in guard pos
    } else if new_pos == '.' {
        updated_count += 1;
    }
    map[pos.1][pos.0] = guard_replace;
    map[guard_y][guard_x] = guard;
    (guard_x, guard_y, updated_count, update_path(guard, new_pos))
}

fn detect_loop(guard: char, new: char) -> bool {
    guard == new ||
    new == '+' ||
    (guard == '>' || guard == '<') && new == '-' || 
    (guard == '^' || guard == 'v') && new == '|' ||
    (guard == '<' || guard == '^' || guard == '>') && new == '@' ||
    (guard == '^' || guard == '>' || guard == 'v') && new == '$' ||
    (guard == '>' || guard == 'v' || guard == '<') && new == '%' ||
    (guard == 'v' || guard == '<' || guard == '^') && new == '&'
}

// <^> -> @
// ^>v -> $
// >v< -> %
// v<^ -> &

fn update_path(guard: char, new: char) -> char {
    if (guard == '^' && new == 'v') || (guard == 'v' && new == '^') {
        return '|';
    } else if (guard == '<' && new == '>') || (guard == '>' && new == '<') {
        return '-';
    } else if new == '|'  {
        if guard == '^' || guard == 'v' { return '|' } // this shouldn't happen
        else if guard == '<' { return '&' } // v<^
        else if guard == '>' { return  '$' } // ^>v
    } else if new == '-' {
        if guard == '<' || guard == '>' { return '-' } // this shouldn't happen
        else if guard == '^' { return '@' } // <^>
        else if guard == 'v' { return '%' } // >v<
    } else if (new == '@' && guard == 'v') || 
        (new == '$' && guard == '<') ||
        (new == '%' && guard == '^') ||
        (new == '&' && guard == '>') { return '+' }
    guard
}

fn get_next(guard: &char) -> char {
    match guard {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Guard character {guard} is not an arrow!")
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..map.len() {
        let row = map.get(y).unwrap();
        for x in 0..row.len() {
            let c = *row.get(x).unwrap();
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                return (x, y);
            }
        }
    }
    (usize::MAX, usize::MAX)
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}