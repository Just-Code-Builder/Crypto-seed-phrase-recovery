use anyhow::Result;
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    key::Secp256k1,
    Address, Network,
};
use secp256k1::PublicKey as RawPublicKey;
use std::str::FromStr;

use super::derivation::mnemonic_to_seed_bytes;

pub struct BitcoinDeriver {
    seed: [u8; 64],
}

impl BitcoinDeriver {
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
                let path = DerivationPath::from_str(&format!("m/44'/0'/0'/0/{}", i)).ok()?;
                let child = xpriv.derive_priv(&secp, &path).ok()?;
                let privkey = child.to_priv();
                // Derive compressed public key
                let raw_pubkey = RawPublicKey::from_secret_key(&secp, &privkey.inner);
                let compressed = bitcoin::PublicKey::new(raw_pubkey);
                // p2wpkh requires a compressed key; bitcoin::PublicKey::new always gives compressed
                Address::p2wpkh(&compressed, Network::Bitcoin).ok().map(|a| a.to_string())
            })
            .collect()
    }
}
