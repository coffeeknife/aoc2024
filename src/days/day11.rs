use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

use itertools::Itertools;
use num::BigInt;

static BLINK_COUNT: usize = 25;
static BLINK_COUNT_2: usize = 75;

pub fn day11(input: String) {
    let stones: Vec<BigInt> = parse_input(input);
    let mut stone_freq: HashMap<BigInt, BigInt> = HashMap::new();

    // set up initial frequency map
    for stone in stones {
        stone_freq.entry(stone).and_modify(|v: &mut BigInt| {*v += 1}).or_insert_with(|| BigInt::from(1));
    }

    for i in 0..BLINK_COUNT_2 {
        if i == BLINK_COUNT {
            println!("Part 1 Solution: {}", count_stones(&stone_freq));
        }
        
        let mut updated: HashMap<BigInt, BigInt> = HashMap::new();
        for k in stone_freq.keys() {
            let count = stone_freq.get(k).unwrap();
            if BigInt::from(0).eq(k) {
                updated.entry(BigInt::from(1)).and_modify(|v: &mut BigInt| {*v += count}).or_insert(count.clone());
            } else if k.to_string().len() % 2 == 0 {
                let key_left: BigInt = k / BigInt::from(10).pow(k.to_string().len() as u32 / 2);
                let key_right: BigInt = k % BigInt::from(10).pow(k.to_string().len() as u32 / 2);
                updated.entry(key_left).and_modify(|v: &mut BigInt| {*v += count}).or_insert(count.clone());
                updated.entry(key_right).and_modify(|v: &mut BigInt| {*v += count}).or_insert(count.clone());
            } else {
                updated.entry(k * 2024).and_modify(|v: &mut BigInt| {*v += count}).or_insert(count.clone());
            }
        }
        stone_freq = updated;
    }

    println!("Part 2 Solution: {}", count_stones(&stone_freq));
}

fn count_stones(stones: &HashMap<BigInt, BigInt>) -> BigInt {
    let mut sum: BigInt = BigInt::from(0);
    for stone in stones.keys() {
        sum += stones.get(stone).unwrap();
    }
    sum
}

fn parse_input(input: String) -> Vec<BigInt> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().next().expect("No data in file?").trim().split(" ").map(|x| x.parse::<BigInt>().expect("bruh")).collect_vec()
}