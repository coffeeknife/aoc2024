use std::{fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use progress_bar::{finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Color, Style};

pub fn day9(input: String) {
    println!("[RUNNING DAY 9]");
    let disk: Vec<(usize, usize)> = parse_input(input);

    // part 1
    let mut d_part1: Vec<usize> = flatten_disk(&disk);
    let mut last_set: usize = 0;
    let disk_len: usize = d_part1.len();

    init_progress_bar(disk_len);
    set_progress_bar_action("Solving Pt1", Color::Blue, Style::Bold);
    for j in 1..disk_len + 1 {
        let i: usize = disk_len - j;

        if d_part1[i] == usize::MAX { inc_progress_bar(); continue }

        for k in last_set..i {
            if d_part1[k] == usize::MAX {
                d_part1[k] = d_part1[i];
                d_part1[i] = usize::MAX;
                last_set = k.clone();
                break;
            }
        }
        
        inc_progress_bar();
    }
    finalize_progress_bar();

    println!("Part 1 Solution: {}", disk_checksum(d_part1));

    // PART 2
    let mut disk_new: Vec<(usize, usize)> = disk.clone();
    let mut offset: usize = 0;

    init_progress_bar(disk_new.len());
    set_progress_bar_action("Solving Pt2", Color::Blue, Style::Bold);
    for i in (0..disk_new.len()).rev() {
        inc_progress_bar();
        let entry: (usize, usize) = disk_new[i+offset].clone();
        if entry.0 == usize::MAX { continue }
        for j in 0..i+offset {
            let chk: (usize, usize) = disk_new[j].clone();
            if chk.0 != usize::MAX || chk.1 < entry.1 { continue }
            disk_new[j] = entry;
            disk_new[i+offset] = (usize::MAX, entry.1);
            if chk.1 > entry.1 { disk_new.insert(j+1, (usize::MAX, chk.1 - entry.1)); offset += 1; }
            break;
        }
    }

    finalize_progress_bar();
    println!("Part 2 Solution: {}", disk_checksum(flatten_disk(&disk_new)))

}

fn disk_checksum(flattened: Vec<usize>) -> usize {
    let mut checksum: usize = 0;
    for i in 0..flattened.len() {
        if flattened[i] == usize::MAX { continue; }
        checksum += flattened[i] * i;
    }
    checksum
}

fn flatten_disk(disk: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut flattened: Vec<usize> = Vec::new();
    for entry in disk {
        for _ in 0..entry.1 {
            flattened.push(entry.0)
        }
    }
    flattened
}

// block with id usize::MAX is empty space
fn parse_input(input: String) -> Vec<(usize, usize)> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let diskmap: Vec<usize> = BufReader::new(file).lines().flatten().join("").chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect_vec();

    let mut is_empty_space = false;
    let mut cur_block_id: usize = 0;
    let mut disk: Vec<(usize, usize)> = Vec::new();
    for entry in diskmap {
        if is_empty_space {
            disk.push((usize::MAX, entry));
        } else {
            disk.push((cur_block_id, entry));
            cur_block_id += 1;
        }
        is_empty_space = !is_empty_space
    }

    disk
}