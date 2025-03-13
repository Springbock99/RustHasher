**RustHasher**  
A lightning-fast command-line tool crafted in Rust to generate Ethereum smart contract function selectors. Powered by Keccak-256, RustHasher takes function signatures (e.g., `transfer(address,uint256)`) and delivers precise 4-byte selectors (e.g., `0xa9059cbb`) right in your terminal. Built for blockchain developers, it combines Rust’s performance and safety with a no-nonsense CLI—perfect for quick hashing or integrating into your workflow.

#### Features

- **Keccak-256 Hashing:** Computes Ethereum-compatible 4-byte function selectors.
- **Simple CLI:** Enter a signature, get a selector—no fuss, no frills.
- **Rust-Powered:** Harnesses Rust’s speed and memory safety for reliable results.

#### Usage

1. **Build the project:**
   ```bash
   cargo build
   ```
