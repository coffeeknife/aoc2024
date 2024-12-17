use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use lazy_static::lazy_static;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, set_progress_bar_progress, Color, Style};
use regex::Regex;

#[derive(Copy, Clone)]
struct Robot {
    p: (usize, usize),
    v: (i32, i32)
}

lazy_static!(
    static ref ROBOT: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    // change this for testing vs production
    static ref TILES_X: i32 = 101;
    static ref TILES_Y: i32 = 103;
    static ref SECONDS: usize = 100;
);

// quadrants:
// 0 1
// 2 3

pub fn day14(input: String) {
    let robots: Vec<Robot> = parse_input(input);
    let part1 = robots.iter().map(|x| parse_seconds(x, *SECONDS));

    // quad left min is 0
    let quadleft_max: usize = ((*TILES_X - 1) / 2) as usize - 1;
    let quadright_min: usize = ((*TILES_X + 1) / 2) as usize;
    // quad left max is tiles_x
    let quadtop_max: usize = ((*TILES_Y  - 1) / 2) as usize - 1;
    let quadbot_min: usize = ((*TILES_Y + 1) / 2) as usize;

    let mut quad0_count: u64 = 0;
    let mut quad1_count: u64 = 0;
    let mut quad2_count: u64 = 0;
    let mut quad3_count: u64 = 0;

    for robot in part1 {

        if robot.p.0 <= quadleft_max {
            if robot.p.1 <= quadtop_max {
                quad0_count += 1;
            } else if robot.p.1 >= quadbot_min {
                quad2_count += 1;
            }
        } else if robot.p.0 >= quadright_min {
            if robot.p.1 <= quadtop_max {
                quad1_count += 1;
            } else if robot.p.1 >= quadbot_min {
                quad3_count += 1;
            }
        }
    }

    println!("Part 1 Solution: {}", quad0_count * quad1_count * quad2_count * quad3_count);

    static STEPS: usize = 10000;

    init_progress_bar(STEPS);
    set_progress_bar_action("Solving Pt2", Color::Blue, Style::Bold);
    let mut cur_robots = robots.clone();
    
    for i in 1..STEPS+1 { // cap tries at 10000
        cur_robots = cur_robots.iter().map(|x| parse_seconds(x, 1)).collect_vec();
        let points = cur_robots.iter().map(|x: &Robot| x.p).collect_vec();
        let mut unique_points = points.clone();
        unique_points.sort(); unique_points.dedup();

        if unique_points.len() == points.len() {
            set_progress_bar_progress(STEPS);
            finalize_progress_bar();
            println!("Part 2 Solution: {i}");
            break;
        }

        inc_progress_bar();
    }

}

fn parse_seconds(robot: &Robot, seconds: usize) -> Robot {
    if seconds == 0 { return *robot }

    let mut px_new: i32 = (robot.p.0 as i32 + robot.v.0) % *TILES_X;
    let mut py_new: i32 = (robot.p.1 as i32 + robot.v.1) % *TILES_Y;

    if px_new < 0 { px_new += *TILES_X }
    else if px_new >= *TILES_X { px_new -= *TILES_X }

    if py_new < 0 { py_new += *TILES_Y }
    else if py_new >= *TILES_Y { py_new -= *TILES_Y }

    parse_seconds(&Robot {
        p: (px_new as usize, py_new as usize),
        v: robot.v
    }, seconds - 1)
}

fn parse_input(input: String) -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();

    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();

    for line in lines {
        if let Some(captures) = ROBOT.captures(&line) {
            let (_, [px, py, vx, vy]) = captures.extract();
            robots.push(Robot { 
                p: (
                    px.parse::<usize>().expect("Data format error"),
                    py.parse::<usize>().expect("Data format error")
                ), 
                v: (
                    vx.parse::<i32>().expect("Data format error"),
                    vy.parse::<i32>().expect("Data format error")
                ) 
            });
        } else {
            println!("WARN: line '{line}' formatted incorrectly")
        }
    }

    robots
}