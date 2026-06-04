use anyhow::Result;
use ed25519_dalek::SigningKey;
use hmac::{Hmac, Mac};
use sha2::Sha512;

use super::derivation::mnemonic_to_seed_bytes;

type HmacSha512 = Hmac<Sha512>;

pub struct SolanaDeriver {
    seed: [u8; 64],
}

impl SolanaDeriver {
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        Ok(Self { seed: mnemonic_to_seed_bytes(phrase, "")? })
    }

    pub fn derive_addresses(&self, count: usize) -> Vec<String> {
        (0..count)
            .filter_map(|i| {
                let path = format!("m/44'/501'/{}'/0'", i);
                slip10_derive_ed25519(&self.seed, &path)
                    .map(|key_bytes| {
                        let signing = SigningKey::from_bytes(&key_bytes);
                        bs58::encode(signing.verifying_key().as_bytes()).into_string()
                    })
            })
            .collect()
    }
}

/// SLIP-0010 hierarchical key derivation for ed25519.
/// Returns the 32-byte private key at the given derivation path.
fn slip10_derive_ed25519(seed: &[u8], path: &str) -> Option<[u8; 32]> {
    // Master key from seed
    let mut mac = HmacSha512::new_from_slice(b"ed25519 seed").ok()?;
    mac.update(seed);
    let result = mac.finalize().into_bytes();

    let mut key: [u8; 32] = result[..32].try_into().ok()?;
    let mut chain: [u8; 32] = result[32..].try_into().ok()?;

    // Walk each segment of the derivation path
    for segment in path.trim_start_matches("m/").split('/') {
        let (index_str, hardened) = if let Some(s) = segment.strip_suffix('\'') {
            (s, true)
        } else {
            (segment, false)
        };

        let index: u32 = index_str.parse().ok()?;
        let child_index = if hardened { index | 0x8000_0000 } else { index };

        // Data = 0x00 || key || index (big-endian)
        let mut data = Vec::with_capacity(37);
        data.push(0x00);
        data.extend_from_slice(&key);
        data.extend_from_slice(&child_index.to_be_bytes());

        let mut mac = HmacSha512::new_from_slice(&chain).ok()?;
        mac.update(&data);
        let result = mac.finalize().into_bytes();

        key = result[..32].try_into().ok()?;
        chain = result[32..].try_into().ok()?;
    }

    Some(key)
}
