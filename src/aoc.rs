use std::time::Instant;

use dotenv::dotenv;
use reqwest::{self, Result};

pub enum Part {
    Part1,
    Part2,
}

pub trait Solver {
    fn day(&self) -> i32;
    fn solve_part_1(&self, input: &str) -> String;
    fn solve_part_2(&self, input: &str) -> String;
}

async fn get_input_for_day(day: i32) -> Result<String> {
    let client = reqwest::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);

    dotenv().expect("Failed to load .env file");
    let session_id = std::env::var("SESSION_ID").expect("Failed to get SESSION_ID");

    client
        .get(url)
        .header("Cookie", format!("session={}", session_id))
        .header(
            "User-Agent",
            "https://github.com/FractalBoy/advent-of-code-2023 by reisner.marc@gmail.com",
        )
        .send()
        .await?
        .text()
        .await
}

pub async fn solve(part: Part, solver: &Box<dyn Solver>) -> Result<()> {
    get_input_for_day(solver.day()).await.map(|input| {
        let start = Instant::now();
        let solution = match part {
            Part::Part1 => solver.solve_part_1(&input),
            Part::Part2 => solver.solve_part_2(&input),
        };
        let duration = start.elapsed();
        println!("{}", solution);
        println!("Solution took {} seconds", duration.as_secs_f64());
    })
}
