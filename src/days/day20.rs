use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day20(input: String) {
    let in_map: Vec<Vec<char>> = parse_input(input);

    let mut start: (usize, usize) = (0,0);
    let mut end: (usize, usize) = (0,0);

    let mut map_dist: Vec<Vec<usize>> = vec![vec![in_map.len() * in_map[0].len() + 1; in_map[0].len()]; in_map.len()];

    for y in 0..in_map.len() {
        for x in 0..in_map[0].len() {
            if in_map[y][x] == 'S' {
                start = (x, y);
            } else if in_map[y][x] == 'E' {
                end = (x, y);
            } else if in_map[y][x] == '#' {
                map_dist[y][x] = usize::MAX;
            }
        }
    }

    map_dist[start.1][start.0] = 0;
    for pt in get_adjacent(start, &map_dist) { cascade_update(&mut map_dist, pt, start); }

    let cheat_free: usize = map_dist[end.1][end.0];

    println!("Shortest cheat-free path length: {cheat_free}");

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut step: (usize, usize) = end;

    while map_dist[step.1][step.0] != 0 {
        path.push(step);
        let step_val: usize = map_dist[step.1][step.0];
        let adj: Vec<(usize, usize)> = get_adjacent(step, &map_dist);
        for pt in adj  {
            if map_dist[pt.1][pt.0] == step_val - 1 {
                step = pt;
                break;
            }
        }
    }

    path.push(start);

    // havent worked out why but for some reason you need to add 1 to the cheat size?
    println!("Part 1 Solution: {}", calculate_cheats(&path, 100, 2));
    println!("Part 2 Solution: {}", calculate_cheats(&path, 100, 20));
    /*
    let mut part1: usize = 0;

    init_progress_bar(map_dist.len() * map_dist[0].len());
    set_progress_bar_action("Solving Pt. 1", Color::Blue, Style::Bold);

    for y in 0..map_dist.len() {
        for x in 0..map_dist.len() {
            if map_dist[y][x] == usize::MAX {
                let mut map_test: Vec<Vec<usize>> = map_dist.clone();
                map_test[y][x] = in_map.len() + in_map[0].len() + 1;
                cascade_update(&mut map_test, (x, y), start);
                if cheat_free - map_test[end.1][end.0] >= SAVINGS { part1 += 1; }
            }
            inc_progress_bar();
        }
    }
    finalize_progress_bar();

    println!("Part 1 Solution: {part1}")
    */
}

fn calculate_cheats(path: &Vec<(usize, usize)>, savings: usize, cheat_len: usize) -> usize {
    let mut cheat_savings: usize = 0;
    
    // search by step savings and check for valid cheats
    for i in 0..path.len() - savings + 1 {
        for j in (i + savings)..path.len() {
            let (cheat_start, cheat_end) = (path[i], path[j]);
            let dist_x: usize = if cheat_end.0 > cheat_start.0 { cheat_end.0 - cheat_start.0 } else { cheat_start.0 - cheat_end.0 };
            let dist_y: usize = if cheat_end.1 > cheat_start.1 { cheat_end.1 - cheat_start.1 } else { cheat_start.1 - cheat_end.1 };
            let updated_len = path.len() - (j - i) + dist_x + dist_y;
            if (path.len() - updated_len) >= savings && (dist_x + dist_y) <= cheat_len {
                cheat_savings += 1;
            }
        }
    }

    cheat_savings
}

fn cascade_update(map: &mut Vec<Vec<usize>>, pt: (usize, usize), start: (usize, usize)) {
    if pt == start { return }
    if map[pt.1][pt.0] == usize::MAX { return }

    let adj_val: usize = map[pt.1][pt.0];
    let mut new_min: usize = get_min_val(&map, get_adjacent(pt, map));
    new_min = safe_add(new_min, 1);
    //new_min = max(new_min, adj_val);
    map[pt.1][pt.0] = new_min;

    if new_min != adj_val {
        let new_adj: Vec<(usize, usize)> = get_adjacent(pt, map);
        for p in new_adj { cascade_update(map, p, start); }
    }
}

// debug function
fn _print_map(map: &Vec<Vec<usize>>) {
    for line in map {
        for entry in line {
            if *entry >= usize::MAX { print!("## ") }
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

fn get_adjacent(pt: (usize, usize), map: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut adj: Vec<(usize, usize)> = Vec::new();
    if pt.0 > 0 { adj.push((pt.0 - 1, pt.1)) }
    if pt.1 > 0 { adj.push((pt.0, pt.1 - 1)) }
    if pt.0 < map[0].len() - 1 { adj.push((pt.0 + 1, pt.1)) }
    if pt.1 < map.len() - 1 { adj.push((pt.0, pt.1 + 1)) }
    adj
}

// add without overflowing
fn safe_add(a:usize, b:usize) -> usize {
    if usize::MAX - a < b || usize::MAX - b < a { usize::MAX }
    else { a + b }
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}

