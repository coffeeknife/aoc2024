use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day12(input: String) {
    let mut garden: Vec<Vec<char>> = parse_input(input);
    let mut part1: u64 = 0;
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            let plot_kind = garden[y][x];
            if plot_kind == '#' { continue; }
            let points: Vec<(usize, usize)> = find_plot((x, y), &garden);
            let area: u64 = points.len() as u64;
            let mut perim: u64 = 0;
            for point in &points {
                let adjacent = get_adjacent(*point, &garden);
                if adjacent.len() < 4 {
                    perim += 4 - adjacent.len() as u64;
                }
                for p in adjacent {
                    if garden[p.1][p.0] != plot_kind {
                        perim += 1;
                    }
                }
            }
            for point in &points {
                garden[point.1][point.0] = '#';
            }
            part1 += area * perim;
        }
    }
    println!("Part 1 Solution: {part1}");
}

fn get_adjacent(point: (usize, usize), garden: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    if point.0 > 0 {
        points.push((point.0 - 1, point.1));
    }
    if point.0 < garden[0].len() - 1 {
        points.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        points.push((point.0, point.1 - 1));
    }
    if point.1 < garden.len() - 1 {
        points.push((point.0, point.1 + 1));
    }
    points
}

fn find_plot(point: (usize, usize), garden: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let mut plot = vec![point];
    let plot_type = garden[point.1][point.0];
    loop {
        let mut added_point = false;
        for p in plot.clone() {
            for q in get_adjacent(p, garden) {
                if garden[q.1][q.0] == plot_type && !plot.contains(&q){
                    plot.push(q);
                    added_point = true;
                }
            }
        }
        if !added_point { break; }
    }
    plot
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}