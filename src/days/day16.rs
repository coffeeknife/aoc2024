use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day16(input: String) {
    let raw_map: Vec<Vec<char>> = parse_input(input);
    let mut weight_map: Vec<Vec<(usize, char)>> = vec![vec![(usize::MAX - 1, '.'); raw_map[0].len()]; raw_map.len()];

    let mut start: (usize, usize) = (0,0);
    let mut goal: (usize, usize) = (0,0);

    for y in 0..raw_map.len() {
        for x in 0..raw_map[0].len() {
            let raw: char = raw_map[y][x];
            match raw {
                'S' => {start = (x, y); weight_map[y][x] = (0, '>')},
                '#' => weight_map[y][x] = (usize::MAX, '#'),
                'E' => {goal = (x, y); weight_map[y][x] = (usize::MAX - 1, 'E')}
                _ => continue
            }
        }
    }

    for pt in get_adjacent(start, (weight_map[0].len(), weight_map.len())) {
        cascade_update(&mut weight_map, pt);
    }

    println!("Part 1 Solution: {}", weight_map[goal.1][goal.0].0);
}

fn cascade_update(map: &mut Vec<Vec<(usize, char)>>, pt: (usize, usize)) {
    let cur_cost: usize = map[pt.1][pt.0].0;
    let cur_char: char = map[pt.1][pt.0].1;
    if cur_char == '#' {return} // don't touch walls
    let mut cost_candidates: Vec<(usize, char)> = vec![(usize::MAX, '*')];
    for adj in get_adjacent(pt, (map[0].len(), map.len())) {
        let mut price: usize = map[adj.1][adj.0].0;
        let dir: char = map[adj.1][adj.0].1;
        if adj.0 < pt.0 { // [adj] > [pt]
            if price < usize::MAX - 2001 { price += get_motion_cost('>', dir ) }
            cost_candidates.push((price, '>'))
        } else if adj.0 > pt.0 { // [pt] < [adj]
            if price < usize::MAX - 2001 { price += get_motion_cost('<', dir) }
            cost_candidates.push((price, '<'))
        } else if adj.1 < pt.1 { // [adj] v/ [pt]
            if price < usize::MAX - 2001 { price += get_motion_cost('v', dir) }
            cost_candidates.push((price, 'v'))
        } else { // [pt] /^ [adj]
            if price < usize::MAX - 2001 { price += get_motion_cost('^', dir) }
            cost_candidates.push((price, '^'))
        }
    }

    let mut best_cost: (usize, char) = (usize::MAX, '*');
    for cost in cost_candidates {
        if cost.0 < best_cost.0 {
            best_cost = cost;
        }
    }
    if best_cost.0 != cur_cost || best_cost.1 != cur_char {
        map[pt.1][pt.0] = best_cost;
        for new in get_adjacent(pt, (map[0].len(), map.len())) { cascade_update(map, new); }
    }
}

fn get_motion_cost(a: char, b: char) -> usize {
    if a == b || b == '*' { 1 }
    else if a == '^' && b == 'v' || a == 'v' && b == '^' || a == '<' && b == '>' || a == '>' && b == '<' { 2001 }
    else { 1001 }
}

fn get_adjacent(pt: (usize, usize), dim: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj: Vec<(usize, usize)> = Vec::new();
    if pt.0 > 0 { adj.push((pt.0 - 1, pt.1)) }
    if pt.1 > 0 { adj.push((pt.0, pt.1 - 1)) }
    if pt.0 < dim.0 - 1 { adj.push((pt.0 + 1, pt.1)) }
    if pt.1 < dim.1 - 1 { adj.push((pt.0, pt.1 + 1)) }
    adj
}


fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}