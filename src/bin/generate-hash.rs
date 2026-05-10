//! Password hash generator for CryptoScope admin authentication.
//!
//! Usage:
//! ```bash
//! echo "your-password" | cargo run --bin generate-hash
//! # or
//! cargo run --bin generate-hash -- your-password
//! ```

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use rand::thread_rng;
use std::io::{self, BufRead};

fn main() -> anyhow::Result<()> {
    let password = if let Some(arg) = std::env::args().nth(1) {
        // Password provided as argument
        arg
    } else {
        // Read from stdin (piped input)
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        if let Some(Ok(line)) = lines.next() {
            line.trim().to_string()
        } else {
            eprintln!("Usage:");
            eprintln!("  echo \"your-password\" | cargo run --bin generate-hash");
            eprintln!("  cargo run --bin generate-hash -- your-password");
            eprintln!();
            anyhow::bail!("No password provided");
        }
    };

    if password.is_empty() {
        anyhow::bail!("Password cannot be empty");
    }

    if password.len() < 12 {
        eprintln!(
            "Warning: Password is less than 12 characters. Consider using a longer password."
        );
    }

    // Generate a random salt
    let salt = SaltString::generate(&mut thread_rng());

    // Create Argon2id hasher with recommended parameters
    let argon2 = Argon2::default();

    // Hash the password
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {e}"))?;

    // Output the hash in PHC format
    println!("{hash}");

    eprintln!();
    eprintln!("✅ Password hash generated successfully!");
    eprintln!();
    eprintln!("Add this to your .env file:");
    eprintln!("ADMIN_PASS_HASH={hash}");
    eprintln!();
    eprintln!("⚠️  Store this hash securely. It cannot be reversed to recover the password.");

    Ok(())
}
