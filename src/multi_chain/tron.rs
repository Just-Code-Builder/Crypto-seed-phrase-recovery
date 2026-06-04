/// TRON address derivation.
///
/// TRON uses secp256k1 — same cryptography as Ethereum but:
///   Path:    m/44'/195'/0'/0/index
///   Address: Base58Check( 0x41 || keccak256(pubkey_uncompressed[1..])[12..] )
///
/// Compatible with TronLink, Trust Wallet, imToken.

use anyhow::Result;
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    key::Secp256k1,
    Network,
};
use sha3::{Digest, Keccak256};
use sha2::Sha256;
use std::str::FromStr;

use super::derivation::mnemonic_to_seed_bytes;

pub struct TronDeriver {
    seed: [u8; 64],
}

impl TronDeriver {
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        Ok(Self { seed: mnemonic_to_seed_bytes(phrase, "")? })
    }

    pub fn derive_addresses(&self, count: usize) -> Vec<String> {
        let secp  = Secp256k1::new();
        let xpriv = match Xpriv::new_master(Network::Bitcoin, &self.seed) {
            Ok(k) => k,
            Err(_) => return vec![],
        };

        (0..count)
            .filter_map(|i| {
                let path = DerivationPath::from_str(&format!("m/44'/195'/0'/0/{}", i)).ok()?;
                let child = xpriv.derive_priv(&secp, &path).ok()?;
                let raw_pubkey = secp256k1::PublicKey::from_secret_key(&secp, &child.to_priv().inner);
                Some(pubkey_to_tron_address(&raw_pubkey))
            })
            .collect()
    }

    pub fn private_key_at(&self, index: u32) -> Option<[u8; 32]> {
        let secp  = Secp256k1::new();
        let xpriv = Xpriv::new_master(Network::Bitcoin, &self.seed).ok()?;
        let path  = DerivationPath::from_str(&format!("m/44'/195'/0'/0/{}", index)).ok()?;
        let child = xpriv.derive_priv(&secp, &path).ok()?;
        Some(child.to_priv().inner.secret_bytes())
    }
}

/// Derive a TRON address from a secp256k1 public key.
pub fn pubkey_to_tron_address(pubkey: &secp256k1::PublicKey) -> String {
    let uncompressed = pubkey.serialize_uncompressed(); // 65 bytes, 0x04 prefix
    let hash = Keccak256::digest(&uncompressed[1..]);   // keccak256 of 64 bytes
    let addr_bytes = &hash[12..];                        // last 20 bytes

    // TRON mainnet prefix 0x41
    let mut payload = vec![0x41u8];
    payload.extend_from_slice(addr_bytes);

    base58check_encode(&payload)
}

/// Base58Check encoding (used by Bitcoin and TRON).
pub fn base58check_encode(payload: &[u8]) -> String {
    // Checksum = first 4 bytes of SHA256(SHA256(payload))
    let h1 = Sha256::digest(payload);
    let h2 = Sha256::digest(h1);
    let mut data = payload.to_vec();
    data.extend_from_slice(&h2[..4]);
    base58_encode_raw(&data)
}

const BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn base58_encode_raw(data: &[u8]) -> String {
    let leading_zeros = data.iter().take_while(|&&b| b == 0).count();

    // Treat data as a big-endian big integer, repeatedly divide by 58
    let mut num = data.to_vec();
    let mut out = Vec::new();

    loop {
        // Check if all bytes are zero
        if num.iter().all(|&b| b == 0) {
            break;
        }
        let mut remainder = 0u32;
        let mut new_num = Vec::new();
        for &byte in &num {
            let cur = remainder * 256 + byte as u32;
            let q = cur / 58;
            remainder = cur % 58;
            if !new_num.is_empty() || q > 0 {
                new_num.push(q as u8);
            }
        }
        out.push(BASE58[remainder as usize]);
        num = new_num;
    }

    for _ in 0..leading_zeros {
        out.push(BASE58[0]); // '1'
    }

    out.reverse();
    String::from_utf8(out).unwrap()
}
