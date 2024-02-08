# BIP39 XOR

For a given BIP39 seed it generates two other seeds that can be used to recover the original seed by XORing them together.

## Usage

1. Make sure you have Rust installed.
2. Clone this repository.
3. Run `cargo build` in the repository directory.

## Generation

Run `cargo run -- generate --entropy 1010101....010` in the repository directory.

If no entropy is provided then it'll be generated randomly.

## Recovery process

Run `cargo run -- recover --part1 "insert_part1_here" --part2 "insert_part2_here"` in the repository directory.

