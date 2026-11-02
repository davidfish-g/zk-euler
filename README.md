# ZK Verified Project Euler Solutions in Rust

My [Project Euler](https://projecteuler.net) solutions in Rust. The code is encrypted, and each answer comes with a zero-knowledge proof from [RISC Zero](https://risczero.com)'s zkVM that the committed code produced it.

## Requirements

- [Rust](https://rustup.rs)
- [RISC Zero toolchain](https://dev.risczero.com/api/zkvm/install)

## Usage

```sh
cargo run -p verify -- <n|all>                # verify proofs; needs only proofs/
cargo run -p project-euler --release -- <n>   # run a solution and time it
cargo run -p host -- <n|all>                  # generate proofs
```

Only some problems have proofs, since many take hours to generate (see the table below). Changing the guest program, including adding a solution, invalidates every existing proof.

## Adding a problem

Add `src/problems/p###.rs` (zero-padded) exporting `pub fn solve() -> String`, or `pub fn solve_with_input(input: &str) -> String` with its data in `inputs/p###.txt`. The build script picks it up automatically.

## Proof generation times

| Problem | Rough proof time (m3 pro) |
| -------- | -------- |
| 1 | <10s |
| 2 | <10s |
| 3 | ~5h |
| 4 | not generated yet (tried for 6 hours and it didn't finish) |
| 5 | not generated yet |
| 6 | <10s |
| 7 | not generated yet |
| 8 | not generated yet |
| 9 | not generated yet |
| 10 | not generated yet |
| 11 | not generated yet |
| 12 | not generated yet |
| 13 | not generated yet |
| 14 | not generated yet |
| 15 | not generated yet |
| 16 | not generated yet |
| 17 | not generated yet |
| 18 | not generated yet |
| 21 | not generated yet |
| 22 | not generated yet |
| 23 | not generated yet |
| 27 | not generated yet |
| 34 | not generated yet |
| 35 | not generated yet |
| 50 | not generated yet |
| 67 | not generated yet |
| 92 | not generated yet |


## Project structure

```
zk-euler/
├── proofs/                 # Cryptographic receipts
│   ├── image_id.json       # Hash of the guest program
│   └── p001.bin ...        # STARK proofs for each problem
├── verify/                 # Public verifier
│   └── src/main.rs
├── host/                   # Proof generator (encrypted)
│   └── src/main.rs
├── methods/                # zkVM guest build infrastructure
│   └── guest/src/main.rs   # Guest entry point (encrypted)
├── src/                    # Solution code (encrypted)
│   ├── main.rs             # CLI: cargo run -p project-euler -- <n>
│   ├── lib.rs
│   ├── utils/              # Shared utilities
│   │   ├── primes.rs
│   │   ├── digits.rs
│   │   └── math.rs
│   └── problems/           
└── inputs/                 # Problem input data files
```

