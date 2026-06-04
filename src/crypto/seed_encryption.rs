use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::Result;
use zeroize::Zeroizing;

use super::kdf::derive_key_from_password;

pub struct SecureSeed(Zeroizing<String>);

impl SecureSeed {
    pub fn new(phrase: &str) -> Self {
        Self(Zeroizing::new(phrase.to_string()))
    }

    pub fn phrase(&self) -> &str {
        &self.0
    }
}

pub fn encrypt_seed(phrase: &str, password: &str) -> Result<Vec<u8>> {
    let key_bytes = derive_key_from_password(password, &random_salt())?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, phrase.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    let mut output = nonce.to_vec();
    output.extend_from_slice(&ciphertext);
    Ok(output)
}

pub fn decrypt_seed(encrypted: &[u8], password: &str, salt: &[u8]) -> Result<SecureSeed> {
    let key_bytes = derive_key_from_password(password, salt)?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    if encrypted.len() < 12 {
        return Err(anyhow::anyhow!("Invalid encrypted data"));
    }

    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    let phrase = String::from_utf8(plaintext)?;
    Ok(SecureSeed::new(&phrase))
}

fn random_salt() -> Vec<u8> {
    use rand::RngCore;
    let mut salt = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}
