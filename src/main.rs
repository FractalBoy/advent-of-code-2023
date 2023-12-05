mod aoc;
mod day1;
mod day2;
mod day3;
mod day4;

use aoc::Solver;
use clap::Parser;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
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
