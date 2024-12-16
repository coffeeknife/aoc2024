use clap::Parser;

mod days;

#[derive(Parser)]
struct Cli {
    day: u64,
    input: String
}

fn main() {
    let args = Cli::parse();
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
        _ => {
            println!("Day not recognized or implemented");
        }
    }
}
