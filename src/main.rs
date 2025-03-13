use sha3::{Digest, Keccak256};
use std::io;

fn main() {
    println!("Welcome to RustHasher");
    println!("Enter a smart contract function signer (e.g.., transfer(address, uint256)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();

    let selector = &result[..4];
    let hex_selector = hex::encode(selector);

    println!("Function Selector (Keccak-256) :0x{}", hex_selector);


}
