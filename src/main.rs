use advent_of_code_2023::day1;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: u8,
    part: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    match args.day {
        1 => match args.part {
            1 => {
                const DAY_1_PART_1_DATA: &'static str = include_str!("../data/1-1.txt");
                println!("{}", day1::part1::solve(DAY_1_PART_1_DATA)?);
            }
            2 => {
                const DAY_1_PART_2_DATA: &'static str = include_str!("../data/1-2.txt");
                println!("{}", day1::part2::solve(DAY_1_PART_2_DATA)?);
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    Ok(())
}
