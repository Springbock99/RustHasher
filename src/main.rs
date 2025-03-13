use sha3::{Digest, Keccak256};
use std::io;

fn main() {
    println!("Welcome to RustHasher");
    println!("Enter a smart contract function signature (e.g., transfer(address,uint256))");
    println!("Note: Enter the full signature on one line, without 'function' prefix.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    println!("Hashing function signature: {}", input); // No quotes in debug output

    // Hash the function signature
    let mut hasher = Keccak256::new();
    hasher.update(input.as_bytes()); // Explicitly convert to bytes
    let result = hasher.finalize();

    let selector = &result[..4];
    let hex_selector = hex::encode(selector);
    let full_hash = hex::encode(&result); 

    println!("Full Hash (Keccak-256): 0x{}", full_hash);
    println!("Function Selector (first 4 bytes): 0x{}", hex_selector);
}