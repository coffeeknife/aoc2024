use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day8(input: String) {  // TODO rewrite by just recording inside the string like day 6
    println!("[RUNNING DAY 8]");
    let (dimensions, antennae) = parse_input(input);
    let mut anode_map: Vec<Vec<bool>> = vec![vec![false; dimensions.0]; dimensions.1];

    for (_, points) in antennae.iter() {
        let mut rem = points.clone();
        while let Some(cur) = rem.pop() {
            if rem.len() == 0 { break; }
            for check in &rem {
                let (point0, point1) = get_antinodes(cur, *check);
                if is_good(point0, dimensions) {
                    anode_map[point0.1 as usize][point0.0 as usize] = true;
                }
                if is_good(point1, dimensions) {
                    anode_map[point1.1 as usize][point1.0 as usize] = true;
                }
            }
        }
    }
    
    let mut part1: u64 = 0;
    for row in anode_map {
        for i in row {
            if i { part1 += 1 }
        }
    }
    println!("Part 1 Solution: {part1}");

    let mut anode_map2: Vec<Vec<bool>> = vec![vec![false; dimensions.0]; dimensions.1];

    for (_, points) in antennae.iter() {
        let mut rem = points.clone();
        while let Some(cur) = rem.pop() {
            if rem.len() == 0 { break; }
            for check in &rem {
                let mut step_x: i64 = (check.0 as i64) - (cur.0 as i64);
                let mut step_y: i64 = (check.1 as i64) - (cur.1 as i64);
                let div = gcd(step_x.abs() as usize, step_y.abs() as usize) as i64;
                step_x = step_x / div; step_y = step_y / div;
                let mut cur_pos = (cur.0 as i64, cur.1 as i64);
                while is_good(cur_pos, dimensions) {
                    anode_map2[cur_pos.1 as usize][cur_pos.0 as usize] = true;
                    cur_pos = (cur_pos.0 + step_x, cur_pos.1 + step_y)
                }
                cur_pos = (cur.0 as i64, cur.1 as i64);
                while is_good(cur_pos, dimensions) {
                    anode_map2[cur_pos.1 as usize][cur_pos.0 as usize] = true;
                    cur_pos = (cur_pos.0 - step_x, cur_pos.1 - step_y)
                }
            }
        }
    }

    let mut part2: u64 = 0;
    for row in anode_map2 {
        for i in row {
            if i { part2 += 1 }
        }
    }
    println!("Part 2 Solution: {part2}");
}

fn is_good(point: (i64, i64), dimensions: (usize, usize)) -> bool {
    if point.0 < 0 || point.1 < 0 || point.0 >= dimensions.0 as i64 || point.1 >= dimensions.1 as i64 {
        false
    } else {
        true
    }
}

fn gcd(mut a:usize, mut b:usize) -> usize{
    if a==b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
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