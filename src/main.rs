use advent_of_code_2023::*;
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
        2 => match args.part {
            1 => {
                const DAY_2_PART_1_DATA: &'static str = include_str!("../data/2-1.txt");
                // 12 red cubes, 13 green cubes, and 14 blue cubes
                println!("{}", day2::part1::solve(12, 13, 14, DAY_2_PART_1_DATA,)?);
            }
            2 => {
                const DAY_2_PART_1_DATA: &'static str = include_str!("../data/2-1.txt");
                println!("{}", day2::part2::solve(DAY_2_PART_1_DATA,)?);
            }
            _ => unimplemented!(),
        },
        3 => match args.part {
            1 => {
                const DAY_3_PART_1_DATA: &'static str = include_str!("../data/3-1.txt");
                println!("{}", day3::part1::solve(DAY_3_PART_1_DATA,));
            }
            2 => {
                const DAY_3_PART_1_DATA: &'static str = include_str!("../data/3-1.txt");
                println!("{}", day3::part2::solve(DAY_3_PART_1_DATA,));
            }
            _ => unimplemented!(),
        },
        4 => match args.part {
            1 => {
                const DAY_4_PART_1_DATA: &'static str = include_str!("../data/4-1.txt");
                println!("{}", day4::part1::solve(DAY_4_PART_1_DATA)?);
            }
            2 => unimplemented!(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    Ok(())
}
