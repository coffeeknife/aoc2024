use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use count_digits::CountDigits;
use itertools::Itertools;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color, Style};

static BLINK_COUNT: usize = 25;
static BLINK_COUNT_2: usize = 75;

pub fn day11(input: String) {
    let mut stones: Vec<u64> = parse_input(input);
    init_progress_bar(BLINK_COUNT_2);
    set_progress_bar_action("Solving...", Color::Blue, Style::Bold);
    let mut part1: usize = 0;
    for i in 0..BLINK_COUNT_2 {
        let mut new_stones: Vec<u64> = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1)
            } else if stone.count_digits() % 2 == 0 {
                let stringy = stone.to_string();
                let (first, second) = stringy.split_at(stone.count_digits() / 2);
                new_stones.push(first.parse::<u64>().expect("bruh"));
                new_stones.push(second.parse::<u64>().expect("bruh"));
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
        if i == BLINK_COUNT - 1 { part1 = stones.len(); }
        inc_progress_bar();
    }
    finalize_progress_bar();
    println!("Part 1 Solution: {}", part1);
    println!("Part 2 Solution: {}", stones.len());
}

fn parse_input(input: String) -> Vec<u64> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().next().expect("No data in file?").trim().split(" ").map(|x| x.parse::<u64>().expect("bruh")).collect_vec()
}