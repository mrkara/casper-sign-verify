use anyhow::{anyhow, Context, Result};
use casper_types::{AsymmetricType, PublicKey, SecretKey};
use std::fs;
use std::path::Path;

const ED25519_TAG: u8 = 1;
const SECP256K1_TAG: u8 = 2;

/// Reads a secret key from a PEM file
pub fn read_secret_key<P: AsRef<Path>>(path: P) -> Result<SecretKey> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(anyhow!(
            "Couldn't access your private key at this location: {}\n\
             Please make sure your secret_key.pem file is at the given location and is accessible by the current user.",
            path.display()
        ));
    }

    casper_types::crypto::SecretKey::from_file(path).with_context(|| format!("Failed to read secret key from {}", path.display()))
}

/// Formats a public key as hex string with the appropriate algorithm prefix
pub fn format_public_key_hex(public_key: &PublicKey) -> String {
    match public_key {
        PublicKey::System => unreachable!("System keys cannot be formatted as hex"),
        PublicKey::Ed25519(_) => {
            let mut bytes = vec![ED25519_TAG];
            let key_bytes = casper_types::bytesrepr::ToBytes::to_bytes(public_key).unwrap();
            bytes.extend_from_slice(&key_bytes[1..]);
            hex::encode(bytes)
        }
        PublicKey::Secp256k1(_) => {
            // PublicKey::to_bytes() is available in casper_types
            let mut bytes = vec![SECP256K1_TAG];
            // Get the raw bytes without the tag that casper_types prepends
            let key_bytes = casper_types::bytesrepr::ToBytes::to_bytes(public_key).unwrap();
            bytes.extend_from_slice(&key_bytes[1..]);
            hex::encode(bytes)
        }
        _ => unreachable!("Unsupported key type"),
    }
}

/// Parses a public key from a hex string with an algorithm prefix
pub fn parse_public_key_hex(hex_str: &str) -> Result<PublicKey> {
    let bytes = hex::decode(hex_str).context("Invalid hex string for public key")?;
    
    if bytes.is_empty() {
        return Err(anyhow!("Public key hex string is empty"));
    }

    let tag = bytes[0];
    let key_bytes = &bytes[1..];

    match tag {
        ED25519_TAG => {
            if key_bytes.len() != 32 {
                return Err(anyhow!("Invalid length for Ed25519 public key"));
            }
            PublicKey::ed25519_from_bytes(key_bytes)
                .map_err(|e| anyhow!("Failed to parse Ed25519 public key: {:?}", e))
        }
        SECP256K1_TAG => {
            if key_bytes.len() != 33 {
                return Err(anyhow!("Invalid length for Secp256k1 public key"));
            }
            PublicKey::secp256k1_from_bytes(key_bytes)
                .map_err(|e| anyhow!("Failed to parse Secp256k1 public key: {:?}", e))
        }
        _ => Err(anyhow!("Unknown algorithm tag: {}", tag)),
    }
}

/// Reads a public key hex string from the default location
pub fn read_public_key_hex_from_default() -> Result<String> {
    let path = "/etc/casper/validator_keys/public_key_hex";
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|_| anyhow!(
            "Couldn't access your public key hex at this location: {}\n\
             Please make sure your public_key_hex file is at the given location and is accessible by the current user.",
            path
        ))
}
