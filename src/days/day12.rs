use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color, Style};

pub fn day12(input: String) {
    let mut garden: Vec<Vec<char>> = parse_input(input);
    let mut part1: usize = 0;
    let mut part2: usize = 0;

    init_progress_bar(garden.len() * garden[0].len());
    set_progress_bar_action("Solving...", Color::Blue, Style::Bold);
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            inc_progress_bar();
            let plot_kind = garden[y][x];
            if plot_kind == '#' { continue; }

            let points: Vec<(usize, usize)> = find_plot((x, y), &garden);
            let area: usize = points.len() as usize;

            let mut perim: usize = 0;
            let mut corner_count: usize = 0;

            for point in &points {

                // part 1 math
                let adjacent: Vec<(usize, usize)> = get_adjacent(*point, &garden);
                if adjacent.len() < 4 {
                    perim += 4 - adjacent.len() as usize;
                }
                for p in adjacent {
                    if garden[p.1][p.0] != plot_kind {
                        perim += 1;
                    }
                }
                // part 2 math, points labeled by cardinal direction
                let point_w: char = get_point(&garden, (point.0 as i32 - 1, point.1 as i32));
                let point_nw: char = get_point(&garden, (point.0 as i32 - 1, point.1 as i32 - 1));
                let point_n: char = get_point(&garden, (point.0 as i32, point.1 as i32 - 1));

                // NW corner
                if check_corner(point_w, point_n, point_nw, plot_kind) { 
                    corner_count += 1 
                }

                let point_ne: char = get_point(&garden, (point.0 as i32 + 1, point.1 as i32 - 1));
                let point_e: char = get_point(&garden, (point.0 as i32 + 1, point.1 as i32));

                // NE corner
                if check_corner(point_n, point_e, point_ne, plot_kind) { corner_count += 1 }

                let point_se: char = get_point(&garden, (point.0 as i32 + 1, point.1 as i32 + 1));
                let point_s: char = get_point(&garden, (point.0 as i32, point.1 as i32 + 1));

                // SE corner
                if check_corner(point_s, point_e, point_se, plot_kind) { corner_count += 1 }

                let point_sw: char = get_point(&garden, (point.0 as i32 - 1, point.1 as i32 + 1));

                // SW corner
                if check_corner(point_s, point_w, point_sw, plot_kind) { corner_count += 1 }
            }
            for point in &points {
                garden[point.1][point.0] = '#';
            }

            part1 += area * perim;
            part2 += area * corner_count;
        }
    }

    finalize_progress_bar();

    println!("Part 1 Solution: {part1}");
    println!("Part 2 Solution: {part2}");
}

fn check_corner(side1: char, side2: char, diagonal: char, plot: char) -> bool {
    if side1 != plot && side2 != plot { true }
    else { side1 == side2 && side1 == plot && diagonal != plot }
}

// safely get point or bogus value if out of bounds
fn get_point(map: &Vec<Vec<char>>, pt: (i32, i32)) -> char {
    if pt.0 < 0 || pt.0 >= map[0].len() as i32 || pt.1 < 0 || pt.1 >= map.len() as i32 { '#' }
    else { map[pt.1 as usize][pt.0 as usize] }
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