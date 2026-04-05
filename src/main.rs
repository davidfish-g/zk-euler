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

    let answer = match problem {
        1 => problems::p001::solve(),
        2 => problems::p002::solve(),
        3 => problems::p003::solve(),
        4 => problems::p004::solve(),
        5 => problems::p005::solve(),
        6 => problems::p006::solve(),
        7 => problems::p007::solve(),
        8 => problems::p008::solve(),
        9 => problems::p009::solve(),
        10 => problems::p010::solve(),
        11 => problems::p011::solve(),
        12 => problems::p012::solve(),
        13 => problems::p013::solve(),
        14 => problems::p014::solve(),
        15 => problems::p015::solve(),
        16 => problems::p016::solve(),
        17 => problems::p017::solve(),
        18 => problems::p018::solve(),
        21 => problems::p021::solve(),
        22 => problems::p022::solve(),
        23 => problems::p023::solve(),
        27 => problems::p027::solve(),
        34 => problems::p034::solve(),
        35 => problems::p035::solve(),
        50 => problems::p050::solve(),
        67 => problems::p067::solve(),
        92 => problems::p092::solve(),
        _ => {
            eprintln!("Problem {} not implemented", problem);
            std::process::exit(1);
        }
    };

    let elapsed = start.elapsed();
    println!("{}", answer);
    println!("That took {} ms", elapsed.as_millis());
}
