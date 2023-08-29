extern crate argon2;

use argon2::{Config, Variant, Version, Error};
use rand::Rng;

pub fn validate_password(password: &str) -> bool {
    // Convert password to ascii
    let password = password.as_bytes();

    // Check if password is between 8 and 64 characters
    if password.len() < 8 || password.len() > 64 {
        return false;
    }

    // Check if password contains at least one uppercase letter
    if !password.iter().any(|&c| c.is_ascii_uppercase()) {
        return false;
    }

    // Check if password contains at least one lowercase letter
    if !password.iter().any(|&c| c.is_ascii_lowercase()) {
        return false;
    }

    // Check if password contains at least one number
    if !password.iter().any(|&c| c.is_ascii_digit()) {
        return false;
    }

    // Check if password contains at least one special character
    if !password.iter().any(|&c| c.is_ascii_punctuation()) {
        return false;
    }

    // Password is valid
    true
}

// Hash password
pub fn hash_password(password: &str) -> Result<String, Error> {
    // Validate password
    if !validate_password(password) {
        return Err(Error::IncorrectType);
    }

    // Convert password to ascii
    let password = password.as_bytes();

    // Generate salt
    let salt = rand::thread_rng().gen::<[u8; 32]>();

    // Generate hash
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        secret: &[],
        ad: &[],
        hash_length: 32,
    };

    // Hash password
    let hash = argon2::hash_encoded(password, &salt, &config)?;

    // Return hash
    Ok(hash)
}

// Verify password
pub fn verify_password(hash: &str, password: &str) -> bool {
    // Verify password
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}