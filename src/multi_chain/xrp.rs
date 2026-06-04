/// XRP (Ripple) address derivation.
/// Path:    m/44'/144'/0'/0/index
/// Key:     secp256k1
/// Address: Base58Check with XRP alphabet, SHA256+RIPEMD160 of compressed pubkey

use anyhow::Result;
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    key::Secp256k1,
    Network,
};
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;
use std::str::FromStr;

use super::derivation::mnemonic_to_seed_bytes;

pub struct XrpDeriver {
    seed: [u8; 64],
}

impl XrpDeriver {
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
                let path = DerivationPath::from_str(&format!("m/44'/144'/0'/0/{}", i)).ok()?;
                let child = xpriv.derive_priv(&secp, &path).ok()?;
                let raw_pubkey = secp256k1::PublicKey::from_secret_key(&secp, &child.to_priv().inner);
                let compressed = raw_pubkey.serialize(); // 33 bytes
                Some(pubkey_to_xrp_address(&compressed))
            })
            .collect()
    }
}

fn pubkey_to_xrp_address(compressed_pubkey: &[u8]) -> String {
    // SHA256 → RIPEMD160
    let sha_hash  = Sha256::digest(compressed_pubkey);
    let ripe_hash = Ripemd160::digest(sha_hash);

    // XRP payload: 0x00 || RIPEMD160_hash (21 bytes)
    let mut payload = vec![0x00u8];
    payload.extend_from_slice(&ripe_hash);

    xrp_base58check(&payload)
}

/// XRP uses a custom Base58 alphabet (different from Bitcoin).
const XRP_ALPHABET: &[u8] = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

fn xrp_base58check(payload: &[u8]) -> String {
    let h1 = Sha256::digest(payload);
    let h2 = Sha256::digest(h1);
    let mut data = payload.to_vec();
    data.extend_from_slice(&h2[..4]);

    let leading_zeros = data.iter().take_while(|&&b| b == 0).count();
    let mut num = data.clone();
    let mut out = Vec::new();

    loop {
        if num.iter().all(|&b| b == 0) { break; }
        let mut remainder = 0u32;
        let mut new_num = Vec::new();
        for &byte in &num {
            let cur = remainder * 256 + byte as u32;
            let q = cur / 58;
            remainder = cur % 58;
            if !new_num.is_empty() || q > 0 { new_num.push(q as u8); }
        }
        out.push(XRP_ALPHABET[remainder as usize]);
        num = new_num;
    }

    for _ in 0..leading_zeros {
        out.push(XRP_ALPHABET[0]);
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}
