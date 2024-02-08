use core::panic;

use bip39::Mnemonic;
use clap::{Parser, Subcommand};

const WORDS: usize = 24;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new mnemonic, split it into two parts and encrypt each part with a password
    Generate {
        /// Etropy as a string of bits, 256 bits, will be generated if not provided
        #[arg(long)]
        entropy: Option<String>,
    },
    /// Recover a mnemonic from two encrypted parts
    Recover {
        #[arg(long)]
        part1: String,
        #[arg(long)]
        part2: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate { entropy }) => {
            let (mnemonic, part1, part2) = generate(entropy);
            println!("Mnemonic: {}", mnemonic);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        }
        Some(Commands::Recover { part1, part2 }) => {
            let mnemonic = recover(&part1, &part2);
            println!("Mnemonic: {}", mnemonic);
        }
        None => println!("No command provided"),
    }
}

/// Generate a new mnemonic. Create two new 264 bit entropy strings that when XORed yield the original entropy.
fn generate(entropy: Option<String>) -> (Mnemonic, Mnemonic, Mnemonic) {
    let mnemonic = if let Some(entropy) = entropy {
        mnemonic_from_bit_string(entropy)
    } else {
        Mnemonic::generate(WORDS).expect("Failed to generate mnemonic")
    };

    let mnemonic_bits_string = mnemonic_to_bit_string(&mnemonic);
    let part1_mnemonic = Mnemonic::generate(WORDS).expect("Failed to generate mnemonic");
    let part1_bits = mnemonic_to_bit_string(&part1_mnemonic);
    let part2_bits = xor_strings(&mnemonic_bits_string, &part1_bits);

    let part2_mnemonic = mnemonic_from_bit_string(part2_bits);

    (mnemonic, part1_mnemonic, part2_mnemonic)
}

fn recover(part1: &str, part2: &str) -> Mnemonic {
    let part1_mnemonic =
        Mnemonic::parse_in(bip39::Language::English, part1).expect("Invalid mnemonic");
    let part2_mnemonic =
        Mnemonic::parse_in(bip39::Language::English, part2).expect("Invalid mnemonic");

    let part1_bits = mnemonic_to_bit_string(&part1_mnemonic);
    let part2_bits = mnemonic_to_bit_string(&part2_mnemonic);

    let mnemonic_bits_string = xor_strings(&part1_bits, &part2_bits);
    mnemonic_from_bit_string(mnemonic_bits_string)
}

fn mnemonic_from_bit_string(entropy: String) -> Mnemonic {
    if entropy.len() != 256 {
        panic!(
            "Invalid entropy length, expected 256 bits, got {} bits",
            entropy.len()
        );
    }
    // convert a string of bits into an array of bytes
    let bytes = (0..256)
        .step_by(8)
        .map(|i| u8::from_str_radix(&entropy[i..i + 8], 2).expect("Invalid entropy"))
        .collect::<Vec<u8>>();
    Mnemonic::from_entropy(&bytes).expect("Invalid entropy")
}

fn mnemonic_to_bit_string(mnemonic: &Mnemonic) -> String {
    mnemonic
        .to_entropy()
        .iter()
        .fold(String::new(), |acc, &byte| acc + &format!("{:08b}", byte))
}

fn xor_strings(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| {
            let val1 = c1 as u8 - '0' as u8;
            let val2 = c2 as u8 - '0' as u8;
            ((val1 ^ val2) + '0' as u8) as char
        })
        .collect()
}
