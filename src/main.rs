mod aoc;
mod day1;

use aoc::Solver;
use clap::Parser;
use day1::Day1;
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

    let dispatch_table = HashMap::from([(1, Day1::new())]);

    if let Some(solver) = dispatch_table.get(&args.day) {
        let solver: Box<&dyn Solver> = Box::new(solver as &dyn Solver);

        if args.part_1 {
            aoc::solve(aoc::Part::Part1, &solver).await?;
        }

        if args.part_2 {
            aoc::solve(aoc::Part::Part2, &solver).await?;
        }

        if !args.part_1 && !args.part_2 {
            eprintln!("Neither part 1 nor part 2 specified");
        }
    } else {
        eprintln!("Day {} is not yet implemented.", args.day);
    }

    Ok(())
}
