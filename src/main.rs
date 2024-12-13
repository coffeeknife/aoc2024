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
        _ => {
            println!("Day not recognized or implemented");
        }
    }
}
