# Casper Sign & Verify - Design Document

## Overview

A lightweight Rust tool to sign messages with Casper validator keys and verify signatures. Replaces the Python `casper-testnet-validator-identification` tool with modern, fast Rust implementation supporting both Ed25519 and Secp256k1 keys.

## Key Features

**Dual Key Algorithm Support**: Supports both Ed25519 (tag: `01`) and Secp256k1 (tag: `02`) keys, automatically detected from PEM files.

**Simple CLI Interface**: Two subcommands (`sign` and `verify`) matching the original Python tool's behavior for easy migration.

**Minimal Dependencies**: Uses only essential crates from the Casper ecosystem, stripping away unnecessary RPC and CLI features from casper-client-rs.

**Fast Performance**: Native Rust implementation provides significant performance improvement over Python.

## Architecture

### Core Modules

| Module | Purpose |
|--------|---------|
| `main.rs` | CLI entry point and argument parsing |
| `sign.rs` | Message signing functionality |
| `verify.rs` | Signature verification functionality |
| `key_utils.rs` | Key loading and format handling |
| `error.rs` | Error types and handling |

### Dependencies

**Essential Crates**:
- `casper-types`: Provides `PublicKey`, `SecretKey`, and cryptographic operations
- `hex`: For hex encoding/decoding of public keys and signatures
- `clap`: Command-line argument parsing (optional, can use std::env)
- `anyhow` or custom error handling: Error propagation

**Rationale**: These are the minimal set needed. We avoid:
- `tokio`: Not needed (no async operations)
- `reqwest`: Not needed (no HTTP)
- `serde_json`: Not needed (simple text output)
- Full RPC/verification modules from casper-client-rs

## CLI Interface

### Sign Command

```
casper-sign-verify sign -m <MESSAGE> [-k <SECRET_KEY_PATH>]
```

**Parameters**:
- `-m, --message <MESSAGE>`: Message to sign (required)
- `-k, --key <PATH>`: Path to secret key PEM file (default: `/etc/casper/validator_keys/secret_key.pem`)
- `-h, --help`: Show help

**Output**:
```
Public Key:
 <public_key_hex_with_prefix>
Message:
 <message>
Signature:
 <signature_hex>
```

### Verify Command

```
casper-sign-verify verify -m <MESSAGE> -s <SIGNATURE> [-k <PUBLIC_KEY_HEX>]
```

**Parameters**:
- `-m, --message <MESSAGE>`: Message that was signed (required)
- `-s, --signature <SIGNATURE>`: Hex-encoded signature (required)
- `-k, --key <PUBLIC_KEY_HEX>`: Public key in hex format with algorithm prefix (optional, can read from file)
- `-h, --help`: Show help

**Output**:
```
Public Key:
 <public_key_hex_with_prefix>
Message:
 <message>
Signature:
 <signature_hex>
Verified!
```

Or on failure:
```
Verification failed!
```

## Implementation Strategy

### Phase 1: Project Setup
- Create Cargo project with minimal dependencies
- Set up module structure
- Implement error handling

### Phase 2: Key Handling
- Implement PEM file reading using casper-types
- Support both Ed25519 and Secp256k1
- Extract public key from secret key
- Format public key with algorithm prefix

### Phase 3: Signing
- Implement message signing using casper-types
- Support both key types
- Output in expected format

### Phase 4: Verification
- Implement signature verification
- Support both key types
- Handle public key hex format with prefix

### Phase 5: CLI Integration
- Parse command-line arguments
- Route to sign/verify functions
- Format and display output

## Compatibility Notes

**Algorithm Prefixes**:
- Ed25519: `01` (1 byte)
- Secp256k1: `02` (1 byte)

**Key Format**: PEM-encoded (PKCS#8 standard)

**Signature Format**: Raw hex-encoded bytes

**Backward Compatibility**: Output format exactly matches Python version for drop-in replacement

## Testing Strategy

1. **Unit Tests**: Test each module independently
2. **Integration Tests**: Test sign/verify round-trip
3. **Compatibility Tests**: Verify output matches Python version
4. **Cross-Key Tests**: Test both Ed25519 and Secp256k1
5. **Error Handling**: Test missing files, invalid keys, etc.

## Future Enhancements

- Support for additional key formats (JSON, etc.)
- Batch signing/verification
- Key generation capability
- Integration with Casper node validator setup
