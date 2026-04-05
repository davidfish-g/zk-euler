use std::env;
use std::time::Instant;

mod utils;
mod problems;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <problem_number>", args[0]);
        std::process::exit(1);
    }

    let problem: u32 = args[1].parse().expect("Please provide a valid problem number");

    let start = Instant::now();
    let answer = problems::solve(problem);
    let elapsed = start.elapsed();

    println!("{}", answer);
    println!("That took {} ms", elapsed.as_millis());
}
