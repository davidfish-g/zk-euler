use risc0_zkvm::Receipt;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <problem_number|all>", args[0]);
        eprintln!("\nVerifies zero-knowledge proofs that Project Euler solutions are correct.");
        eprintln!("No solution code is needed — only the proof files in proofs/.");
        std::process::exit(1);
    }

    // Load the image ID (commitment to the secret solution code)
    let id_json = fs::read_to_string("proofs/image_id.json")
        .expect("Could not read proofs/image_id.json — generate proofs first");
    let image_id: [u32; 8] = serde_json::from_str(&id_json)
        .expect("Invalid image_id.json format");

    let problems: Vec<u32> = if args[1] == "all" {
        let mut probs = Vec::new();
        for entry in fs::read_dir("proofs").unwrap() {
            let name = entry.unwrap().file_name().into_string().unwrap();
            if name.starts_with('p') && name.ends_with(".bin") {
                if let Ok(num) = name[1..4].parse::<u32>() {
                    probs.push(num);
                }
            }
        }
        probs.sort();
        probs
    } else {
        vec![args[1].parse().expect("Invalid problem number")]
    };

    let mut passed = 0;
    let mut failed = 0;

    for problem in &problems {
        let path = format!("proofs/p{:03}.bin", problem);
        let receipt_bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("Problem {:>3}: MISSING ({})", problem, path);
                failed += 1;
                continue;
            }
        };

        let receipt: Receipt = match bincode::deserialize(&receipt_bytes) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Problem {:>3}: CORRUPT ({})", problem, e);
                failed += 1;
                continue;
            }
        };

        // Decode the journal (public output)
        let journal = &receipt.journal.bytes;
        let prob_num: u32 = risc0_zkvm::serde::from_slice(&journal[..4]).unwrap();
        let answer: String = risc0_zkvm::serde::from_slice(&journal[4..]).unwrap();

        match receipt.verify(image_id) {
            Ok(()) => {
                println!("Problem {:>3}: {} [VERIFIED]", prob_num, answer);
                passed += 1;
            }
            Err(e) => {
                eprintln!("Problem {:>3}: FAILED ({})", prob_num, e);
                failed += 1;
            }
        }
    }

    println!("\n{} verified, {} failed", passed, failed);
}
