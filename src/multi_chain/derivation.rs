use anyhow::Result;
use bip39::Mnemonic;

pub fn mnemonic_to_seed_bytes(phrase: &str, passphrase: &str) -> Result<[u8; 64]> {
    let mnemonic = Mnemonic::parse_normalized(phrase)
        .map_err(|e| anyhow::anyhow!("Invalid mnemonic: {}", e))?;
    Ok(mnemonic.to_seed(passphrase))
}
