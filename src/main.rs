use anyhow::Result;
use clap::{Parser, Subcommand};

mod key_utils;
mod sign;
mod verify;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sign a message using a secret key
    Sign {
        /// Message to sign
        #[arg(short, long)]
        message: String,

        /// Path to the secret key PEM file
        #[arg(short, long, default_value = "/etc/casper/validator_keys/secret_key.pem")]
        key: String,
    },
    /// Verify a signature
    Verify {
        /// Message that was signed
        #[arg(short, long)]
        message: String,

        /// Signature in hex format
        #[arg(short, long)]
        signature: String,

        /// Public key in hex format (with algorithm prefix)
        #[arg(short, long)]
        key: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sign { message, key } => {
            sign::execute(message, key)?;
        }
        Commands::Verify { message, signature, key } => {
            verify::execute(message, signature, key.as_deref())?;
        }
    }

    Ok(())
}
