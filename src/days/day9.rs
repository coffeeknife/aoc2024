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
    let disk_orig: Vec<String> = disk.clone();

    // part 1
    let original_len: usize = disk.len();
    init_progress_bar(original_len);
    set_progress_bar_action("Solving Part 1", Color::Blue, Style::Bold);
    loop {
        if !disk.contains(&String::from(".")) { break; }
        let mut cur = disk.len() - 1;
        let cur_block = disk[cur].clone();
        while disk[cur] == cur_block && cur > 0 { cur -= 1; inc_progress_bar(); }
        if cur == 0 { break }
        let mut block_len = disk.len() - cur - 1;
        let mut moved_count = 0;
        for i in 0..disk.len() {
            if block_len == 0 { break; }
            if disk[i].eq(".") {
                disk[i] = cur_block.clone();
                block_len -= 1;
                moved_count += 1;
            }
        }
        while (disk[disk.len() - 1].eq(&cur_block) && moved_count > 0) || disk[disk.len() - 1].eq(".") { disk.pop(); moved_count -= 1; inc_progress_bar(); }
        inc_progress_bar();
    }
    finalize_progress_bar();
    let mut part1: u64 = 0;
    for i in 0..disk.len() {
        part1 += i as u64 * disk[i].parse::<u64>().expect("this shouldn't happen");
    }
    println!("Part 1 Solution: {part1}");

    let mut disk_blocks: Vec::<(String, usize)> = Vec::new();
    let mut cur_id: String = disk_orig[0].clone();
    let mut start_index:usize = 0;
    for i in 1..disk_orig.len() {
        if !disk_orig[i].eq(&cur_id) {
            disk_blocks.push((cur_id, i - start_index));
            start_index = i;
            cur_id = disk_orig[i].clone();
        }
    }
    disk_blocks.push((cur_id, disk_orig.len() - start_index));
    let len_orig: usize = disk_blocks.len();
    init_progress_bar(len_orig);
    set_progress_bar_action("Solving Pt2", Color::Blue, Style::Bold);
    for j in 0..len_orig {
        inc_progress_bar();
        let i = len_orig - j - 1;
        let (cur, size) = disk_blocks.get(i).expect("This really shouldn't be empty").clone();
        if cur.eq(".") { continue } // can trim any empty blocks at the end
        let mut pushed: bool = false;
        for k in 0..i {
            let (testcur, testsize) = disk_blocks.get(k).expect("Shoudn't be empty").clone();
            if testsize >= size && testcur.clone().eq(".") && !pushed {
                disk_blocks[k] = (cur.clone(), size);
                disk_blocks[i] = (String::from("."), size);
                if testsize - size > 0 {
                    disk_blocks.insert(k+1, (String::from("."), testsize - size))
                }
                pushed = true;
            }
        }
    }
    finalize_progress_bar();

    let mut part2: u64 = 0;
    let mut disk_string: Vec<String> = Vec::new();
    for block in disk_blocks {
        for _ in 0..block.1 {
            disk_string.push(block.0.clone());
        }
    }

    for i in 0..disk_string.len() {
        if String::from(".").eq(&disk_string[i]) { continue }
        part2 += disk_string[i].parse::<u64>().unwrap() * i as u64;
    }
    println!("Part 2 Solution: {part2}")
}

fn parse_input(input: String) -> String {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().join("")
}