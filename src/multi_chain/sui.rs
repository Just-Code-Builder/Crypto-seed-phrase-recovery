/// SUI address derivation.
/// Path:    SLIP-0010 Ed25519  m/44'/784'/index'/0'/0'
/// Address: "0x" + hex( Blake2b-256( 0x00 || ed25519_pubkey_32_bytes ) )
/// Compatible with Sui Wallet, Suiet, Martian, Ethos.

use anyhow::Result;
use blake2::{Blake2b, Digest};
use ed25519_dalek::SigningKey;
use hmac::{Hmac, Mac};
use sha2::Sha512;

type HmacSha512 = Hmac<Sha512>;
type Blake2b256 = Blake2b<blake2::digest::consts::U32>;

use super::derivation::mnemonic_to_seed_bytes;

pub struct SuiDeriver {
    seed: [u8; 64],
}

impl SuiDeriver {
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        Ok(Self { seed: mnemonic_to_seed_bytes(phrase, "")? })
    }

    pub fn derive_addresses(&self, count: usize) -> Vec<String> {
        (0..count)
            .filter_map(|i| {
                // m/44'/784'/i'/0'/0'
                let path = format!("m/44'/784'/{}'/0'/0'", i);
                slip10_derive_ed25519(&self.seed, &path)
                    .map(|key| signing_key_to_sui_address(&key))
            })
            .collect()
    }
}

/// Derive a SUI address from an Ed25519 signing key.
/// SUI address = 0x + hex( Blake2b-256( flag_byte=0x00 || 32-byte-pubkey ) )
pub fn signing_key_to_sui_address(key_bytes: &[u8; 32]) -> String {
    let signing = SigningKey::from_bytes(key_bytes);
    let pubkey  = signing.verifying_key().to_bytes();

    let mut hasher = Blake2b256::new();
    hasher.update([0x00u8]);   // Ed25519 scheme flag
    hasher.update(pubkey);
    let hash = hasher.finalize();

    format!("0x{}", hex::encode(hash))
}

/// SLIP-0010 hardened derivation for Ed25519 (all segments hardened for SUI).
fn slip10_derive_ed25519(seed: &[u8; 64], path: &str) -> Option<[u8; 32]> {
    let mut mac = HmacSha512::new_from_slice(b"ed25519 seed").ok()?;
    mac.update(seed);
    let result = mac.finalize().into_bytes();

    let mut key:   [u8; 32] = result[..32].try_into().ok()?;
    let mut chain: [u8; 32] = result[32..].try_into().ok()?;

    for segment in path.trim_start_matches("m/").split('/') {
        let (idx_str, hardened) = segment.strip_suffix('\'')
            .map(|s| (s, true))
            .unwrap_or((segment, false));

        let index: u32 = idx_str.parse().ok()?;
        let child_index = if hardened { index | 0x8000_0000 } else { index };

        let mut data = vec![0x00u8];
        data.extend_from_slice(&key);
        data.extend_from_slice(&child_index.to_be_bytes());

        let mut mac = HmacSha512::new_from_slice(&chain).ok()?;
        mac.update(&data);
        let result = mac.finalize().into_bytes();
        key   = result[..32].try_into().ok()?;
        chain = result[32..].try_into().ok()?;
    }

    Some(key)
}
