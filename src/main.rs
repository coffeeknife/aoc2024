use std::time::Instant;

use clap::Parser;

mod days;

#[derive(Parser)]
struct Cli {
    day: u64,
    input: String
}

fn main() {
    let args: Cli = Cli::parse();

    println!("[RUNNING DAY {}]", args.day);
    let start: Instant = Instant::now();

    match args.day {
        1 => days::day1::day1(args.input),
        2 => days::day2::day2(args.input),
        3 => days::day3::day3(args.input),
        4 => days::day4::day4(args.input),
        5 => days::day5::day5(args.input),
        6 => days::day6::day6(args.input),
        7 => days::day7::day7(args.input),
        8 => days::day8::day8(args.input),
        9 => days::day9::day9(args.input),
        10 => days::day10::day10(args.input),
        11 => days::day11::day11(args.input),
        12 => days::day12::day12(args.input),
        13 => days::day13::day13(args.input),
        14 => days::day14::day14(args.input),
        15 => days::day15::day15(args.input),
        16 => days::day16::day16(args.input),
        17 => days::day17::day17(args.input),
        18 => days::day18::day18(args.input),
        19 => days::day19::day19(args.input),
        20 => days::day20::day20(args.input),
        _ => {
            println!("Day not recognized or implemented");
        }
    }

    println!("Day {} executed in {}ms", args.day, start.elapsed().as_millis());
}
