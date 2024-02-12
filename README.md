# BIP39 XOR

For a given BIP39 seed it generates two other seeds that can be used to recover the original seed by XORing them together.

## Usage

1. Make sure you have Rust installed.
2. Clone this repository.
3. Run `cargo build` in the repository directory.

## Generation

`cargo run -- generate --seed 'insert_seed_here'`

or

`cargo run -- generate --entropy 1010101....010`

If neither seed nor entropy is provided then a new seed phrase will be generated randomly.

## Recovery

Run `cargo run -- recover --part1 "insert_part1_here" --part2 "insert_part2_here"` in the repository directory.

