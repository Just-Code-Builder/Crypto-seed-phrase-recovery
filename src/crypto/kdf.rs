use anyhow::Result;
use argon2::{Algorithm, Argon2, Params, Version};

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let params = Params::new(65536, 3, 4, Some(32))
        .map_err(|e| anyhow::anyhow!("Argon2 params error: {}", e))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow::anyhow!("KDF error: {}", e))?;

    Ok(key)
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
}
