use anyhow::Result;
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    key::Secp256k1,
    secp256k1::PublicKey as Secp256k1PublicKey,
    Network,
};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

use super::derivation::mnemonic_to_seed_bytes;

pub struct EthereumDeriver {
    seed: [u8; 64],
}

impl EthereumDeriver {
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        Ok(Self { seed: mnemonic_to_seed_bytes(phrase, "")? })
    }

    pub fn derive_addresses(&self, count: usize) -> Vec<String> {
        let secp = Secp256k1::new();
        let xpriv = match Xpriv::new_master(Network::Bitcoin, &self.seed) {
            Ok(k) => k,
            Err(_) => return vec![],
        };

        (0..count)
            .filter_map(|i| {
                let path = DerivationPath::from_str(&format!("m/44'/60'/0'/0/{}", i)).ok()?;
                let child = xpriv.derive_priv(&secp, &path).ok()?;
                // Extract the raw secret key bytes
                let secret_key = child.to_priv().inner;
                let pubkey = Secp256k1PublicKey::from_secret_key(&secp, &secret_key);
                Some(pubkey_to_checksummed_eth_address(&pubkey))
            })
            .collect()
    }
}

/// Derives the Ethereum address from a secp256k1 public key.
/// Uses uncompressed public key → Keccak256 → last 20 bytes → EIP-55 checksum.
fn pubkey_to_checksummed_eth_address(pubkey: &Secp256k1PublicKey) -> String {
    let uncompressed = pubkey.serialize_uncompressed(); // 65 bytes, starts with 0x04
    let hash = Keccak256::digest(&uncompressed[1..]); // skip 0x04 prefix
    let addr_bytes = &hash[12..]; // last 20 bytes
    eip55_checksum(addr_bytes)
}

/// EIP-55 mixed-case checksum encoding.
fn eip55_checksum(addr_bytes: &[u8]) -> String {
    let addr_hex = hex::encode(addr_bytes);
    let hash = Keccak256::digest(addr_hex.as_bytes());

    let checksummed: String = addr_hex
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_alphabetic() {
                let nibble = (hash[i / 2] >> (if i % 2 == 0 { 4 } else { 0 })) & 0x0f;
                if nibble >= 8 { c.to_uppercase().next().unwrap() } else { c }
            } else {
                c
            }
        })
        .collect();

    format!("0x{}", checksummed)
}
