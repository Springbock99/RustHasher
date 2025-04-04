use sha3::{Digest, Keccak256}; // Import the tools we need for hashing

/// Hashes a function signature and returns the full Keccak-256 hash and 4-byte selector.
///
/// # Arguments
/// * `input` - The function signature (e.g., "transfer(address,uint256)").
///
/// # Returns
/// A tuple containing:
/// * Full Keccak-256 hash as a hex string (e.g., "0x...").
/// * 4-byte function selector as a hex string (e.g., "0xa9059cbb").
///
/// # Examples
/// ```
/// let (full_hash, selector) = rusthasher::hash_signature("transfer(address,uint256)");
/// assert_eq!(selector, "0xa9059cbb");
/// ```
pub fn hash_signature(input: &str) -> (String, String) {
    let mut hasher = Keccak256::new();        // Create a new Keccak-256 hasher
    hasher.update(input.as_bytes());          // Feed the input string into the hasher
    let result = hasher.finalize();           // Get the final 32-byte hash
    let selector = &result[..4];              // Take the first 4 bytes for the selector
    (
        format!("0x{}", hex::encode(&result)), 
        format!("0x{}", hex::encode(selector)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_signature() {
        let (full_hash, selector) = hash_signature("transfer(address,uint256)");
        assert_eq!(selector, "0xa9059cbb");
        assert_eq!(full_hash.len(), 66);
    }
  }