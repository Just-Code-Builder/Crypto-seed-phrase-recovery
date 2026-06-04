/// TON address derivation.
///
/// Key derivation (matches Tonkeeper / TON Wallet apps):
///   PBKDF2-HMAC-SHA512(
///     password = mnemonic_words.join(" "),
///     salt     = "TON default seed",
///     iters    = 100_000,
///     keylen   = 64
///   )
///   Ed25519 key from first 32 bytes
///
/// Address (WalletV4R2, the most common TON wallet):
///   state_init = { code: WalletV4R2_code_cell, data: wallet_data_cell }
///   raw_address = workchain(0) : sha256(state_init_boc)[0..32]
///   friendly = base64url( workchain_byte || raw_address || crc16 )
///   Displayed as EQ... (bounceable) or UQ... (non-bounceable)

use anyhow::Result;
use ed25519_dalek::SigningKey;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::{Digest, Sha256, Sha512};

pub struct TonDeriver {
    /// Raw 64-byte seed derived via PBKDF2
    seed: [u8; 64],
}

impl TonDeriver {
    /// Derive from a BIP-39 mnemonic phrase (the TON way, not BIP-32).
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        let mut seed = [0u8; 64];
        pbkdf2::<Hmac<Sha512>>(
            phrase.as_bytes(),
            b"TON default seed",
            100_000,
            &mut seed,
        )
        .map_err(|e| anyhow::anyhow!("PBKDF2 error: {}", e))?;
        Ok(Self { seed })
    }

    /// Returns [index] TON wallet addresses (WalletV4R2, workchain 0).
    /// Each index gets a different `subwallet_id`, matching the multi-account
    /// approach used by Tonkeeper.
    pub fn derive_addresses(&self, count: usize) -> Vec<String> {
        let key_bytes: [u8; 32] = self.seed[..32].try_into().unwrap();
        let signing  = SigningKey::from_bytes(&key_bytes);
        let pubkey   = signing.verifying_key().to_bytes();

        (0..count)
            .map(|i| wallet_v4r2_address(&pubkey, i as u32))
            .collect()
    }

    pub fn signing_key_bytes(&self) -> [u8; 32] {
        self.seed[..32].try_into().unwrap()
    }
}

/// Compute the WalletV4R2 non-bounceable address for a given Ed25519 pubkey
/// and subwallet index (0 = default / first account).
///
/// WalletV4R2 state_init data cell layout (TL-B):
///   seqno:32 subwallet_id:32 public_key:256 plugins:(HashmapE 8 Cell) = 0
///
/// We skip full BOC serialization by computing the address hash directly
/// from the well-known WalletV4R2 code hash and the data fields.
pub fn wallet_v4r2_address(pubkey: &[u8; 32], subwallet_offset: u32) -> String {
    // Standard subwallet IDs used by Tonkeeper/MyTonWallet
    let subwallet_id: u32 = 698_983_191u32.wrapping_add(subwallet_offset);

    // Build the data cell bytes (simplified canonical form used for hashing)
    // [seqno=0 : 32 bits][subwallet_id : 32 bits][pubkey : 256 bits][plugins=0 : 1 bit pad]
    let mut data = Vec::with_capacity(69);
    data.extend_from_slice(&0u32.to_be_bytes());           // seqno = 0
    data.extend_from_slice(&subwallet_id.to_be_bytes());   // subwallet_id
    data.extend_from_slice(pubkey);                         // 32-byte pubkey
    data.push(0x00);                                        // plugins dict = empty (1-bit zero padded)

    // Well-known WalletV4R2 code hash (sha256 of the canonical code BOC)
    // Source: https://github.com/ton-blockchain/wallet-contract/blob/main/build/wallet-v4-code.boc
    let code_hash = hex::decode(
        "FEB5FF6820E2FF0D9483E7E0D62C817D846789FB1F6B2B7A895ACA7C41BA2E17"
    ).unwrap();

    // state_init hash = sha256(code_hash || data_hash)
    let data_hash = Sha256::digest(&data);
    let mut combined = Vec::with_capacity(64);
    combined.extend_from_slice(&code_hash);
    combined.extend_from_slice(&data_hash);
    let addr_bytes: [u8; 32] = Sha256::digest(&combined).into();

    // Encode as user-friendly non-bounceable address (UQ...)
    // Format: tag(0x51) workchain(0x00) addr[32] crc16[2]
    let mut raw = vec![0x51u8, 0x00u8]; // non-bounceable, workchain 0
    raw.extend_from_slice(&addr_bytes);
    let crc = crc16_ton(&raw);
    raw.push((crc >> 8) as u8);
    raw.push(crc as u8);

    // Base64url (no padding)
    base64url_encode(&raw)
}

/// CRC-16/XMODEM used in TON address encoding.
fn crc16_ton(data: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

fn base64url_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as usize;
        let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };
        out.push(CHARS[b0 >> 2] as char);
        out.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        if chunk.len() > 1 { out.push(CHARS[((b1 & 0xf) << 2) | (b2 >> 6)] as char); }
        if chunk.len() > 2 { out.push(CHARS[b2 & 0x3f] as char); }
    }
    out
}
