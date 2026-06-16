use anyhow::{anyhow, Result};
use casper_types::{PublicKey, bytesrepr::ToBytes};

use crate::key_utils;

const ED25519_TAG: u8 = 1;
const SECP256K1_TAG: u8 = 2;

pub fn execute(message: &str, key_path: &str) -> Result<()> {
    if message.is_empty() {
        return Err(anyhow!("Message can't be empty!"));
    }

    // Read the secret key
    let secret_key = key_utils::read_secret_key(key_path)?;

    // Derive the public key
    let public_key = PublicKey::from(&secret_key);
    let public_key_hex = key_utils::format_public_key_hex(&public_key);

    // Sign the message
    let signature = casper_types::crypto::sign(message.as_bytes(), &secret_key, &public_key);
    let signature_bytes = signature.to_bytes().map_err(|e| anyhow!("Failed to serialize signature: {:?}", e))?;
    
    // Strip the algorithm prefix (first byte) to get the raw 64-byte signature
    let raw_signature_hex = if signature_bytes.len() == 65 && 
        (signature_bytes[0] == ED25519_TAG || signature_bytes[0] == SECP256K1_TAG) {
        // Skip the first byte (algorithm tag) and encode the remaining 64 bytes
        hex::encode(&signature_bytes[1..])
    } else {
        // Fallback: use as-is (shouldn't happen with casper-types)
        hex::encode(&signature_bytes)
    };

    // Print the output in the exact format expected
    println!("Public Key:\n {}", public_key_hex);
    println!("Message:\n {}", message);
    println!("Signature:\n {}", raw_signature_hex);

    Ok(())
}
