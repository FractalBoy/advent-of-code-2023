mod aoc;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc::Solver;
use clap::Parser;
use day1::Day1;
use day10::Day10;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    part_1: bool,
    #[arg(long)]
    part_2: bool,
    #[arg(long, short)]
    day: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let dispatch_table: HashMap<i32, Box<dyn Solver>> = HashMap::from([
        (1, Box::new(Day1::new()) as Box<dyn Solver>),
        (2, Box::new(Day2::new()) as Box<dyn Solver>),
        (3, Box::new(Day3::new()) as Box<dyn Solver>),
        (4, Box::new(Day4::new()) as Box<dyn Solver>),
        (5, Box::new(Day5::new()) as Box<dyn Solver>),
        (6, Box::new(Day6::new()) as Box<dyn Solver>),
        (7, Box::new(Day7::new()) as Box<dyn Solver>),
        (8, Box::new(Day8::new()) as Box<dyn Solver>),
        (9, Box::new(Day9::new()) as Box<dyn Solver>),
        (10, Box::new(Day10::new()) as Box<dyn Solver>),
    ]);

    if let Some(solver) = dispatch_table.get(&args.day) {
        if args.part_1 {
            aoc::solve(aoc::Part::Part1, solver).await?;
        }

        if args.part_2 {
            aoc::solve(aoc::Part::Part2, solver).await?;
        }

        if !args.part_1 && !args.part_2 {
            eprintln!("Neither part 1 nor part 2 specified");
        }
    } else {
        eprintln!("Day {} is not yet implemented.", args.day);
    }

    Ok(())
}
