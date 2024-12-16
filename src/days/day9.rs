use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color, Style};

pub fn day9(input: String) {
    println!("[RUNNING DAY 9]");
    let fs_data: String = parse_input(input);
    let mut disk: Vec<String> = Vec::new();
    let mut is_data: bool = true;
    let mut block_id: u64 = 0;
    for char in fs_data.chars().map(|x| x.to_string().parse::<u64>().expect("Input should be all numbers")) {
        for _ in 0..char {
            if is_data { disk.push(block_id.to_string()); }
            else { disk.push(String::from(".")) }
        }
        if is_data { block_id += 1; }
        is_data = !is_data;
    }

    let original_len: usize = disk.len();
    init_progress_bar(original_len);
    set_progress_bar_action("Solving Part 1", Color::Blue, Style::Bold);
    loop {
        if !disk.contains(&String::from(".")) { break; }
        while disk[disk.len() - 1].eq(".") { disk.pop(); inc_progress_bar(); }
        let mut cur = disk.len() - 1;
        let cur_block = disk[cur].clone();
        while disk[cur] == cur_block && cur > 0 { cur -= 1; inc_progress_bar(); }
        if cur == 0 { break }
        let mut block_len = disk.len() - cur - 1;
        let mut moved_count = 0;
        for i in 0..disk.len() {
            if block_len == 0 { break; }
            if disk[i].eq(".") {
                disk[i] = cur_block.to_string();
                block_len -= 1;
                moved_count += 1;
            }
        }
        for i in disk.len() - moved_count..disk.len() {
            disk[i] = String::from(".");
            inc_progress_bar();
        }
        inc_progress_bar();
    }
    finalize_progress_bar();
    let mut part1: u64 = 0;
    for i in 0..disk.len() {
        part1 += i as u64 * disk[i].parse::<u64>().expect("this shouldn't happen");
    }
    println!("Part 1 Solution: {part1}")
}

fn parse_input(input: String) -> String {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().join("")
}