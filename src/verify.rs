use anyhow::{anyhow, Result};
const ED25519_TAG: u8 = 1;
const SECP256K1_TAG: u8 = 2;

use crate::key_utils;

pub fn execute(message: &str, signature_hex: &str, key_hex: Option<&str>) -> Result<()> {
    if message.is_empty() || signature_hex.is_empty() {
        return Err(anyhow!("Message and signature are required!"));
    }

    // Get the public key hex either from argument or default file
    let public_key_hex = match key_hex {
        Some(hex) => hex.to_string(),
        None => key_utils::read_public_key_hex_from_default()?,
    };

    // Parse the public key
    let public_key = key_utils::parse_public_key_hex(&public_key_hex)?;

    // Parse the signature
    let signature_bytes = hex::decode(signature_hex)
        .map_err(|_| anyhow!("Invalid hex string for signature"))?;
    
    // Handle both raw 64-byte signatures and prefixed 65-byte signatures
    let signature = if signature_bytes.len() == 64 {
        // Raw signature without prefix - add the appropriate prefix based on public key type
        match &public_key {
            casper_types::PublicKey::System => return Err(anyhow!("Cannot verify with system key")),
            casper_types::PublicKey::Ed25519(_) => {
                let mut bytes = vec![ED25519_TAG];
                bytes.extend_from_slice(&signature_bytes);
                casper_types::bytesrepr::FromBytes::from_bytes(&bytes)
                    .map_err(|e| anyhow!("Failed to parse Ed25519 signature: {:?}", e))?.0
            }
            casper_types::PublicKey::Secp256k1(_) => {
                let mut bytes = vec![SECP256K1_TAG];
                bytes.extend_from_slice(&signature_bytes);
                casper_types::bytesrepr::FromBytes::from_bytes(&bytes)
                    .map_err(|e| anyhow!("Failed to parse Secp256k1 signature: {:?}", e))?.0
            }
            _ => return Err(anyhow!("Unsupported public key type")),
        }
    } else if signature_bytes.len() == 65 && (signature_bytes[0] == ED25519_TAG || signature_bytes[0] == SECP256K1_TAG) {
        // Signature with algorithm prefix (for backward compatibility)
        casper_types::bytesrepr::FromBytes::from_bytes(&signature_bytes)
            .map_err(|e| anyhow!("Failed to parse signature: {:?}", e))?.0
    } else {
        return Err(anyhow!("Invalid signature length: expected 64 bytes (raw) or 65 bytes (with prefix), got {}", signature_bytes.len()));
    };

    // Print the inputs
    println!("Public Key:\n {}", public_key_hex);
    println!("Message:\n {}", message);
    println!("Signature:\n {}", signature_hex);

    // Verify the signature
    if casper_types::crypto::verify(message.as_bytes(), &signature, &public_key).is_ok() {
        println!("Verified!");
        Ok(())
    } else {
        println!("Verification failed!");
        // The original python script just printed and didn't exit with error code on failure,
        // but we'll return an error to be more robust while still matching the output
        Err(anyhow!("Verification failed!"))
    }
}
