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
        _ => {
            println!("Day not recognized or implemented");
        }
    }
}
