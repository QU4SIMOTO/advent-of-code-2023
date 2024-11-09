use advent_of_code_2023::day1::solve;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().day {
        1 => {
            const DAY_1_DATA: &'static str = include_str!("../data/1.txt");
            println!("{}", solve(DAY_1_DATA)?);
        }
        _ => unimplemented!(),
    }
    Ok(())
}
