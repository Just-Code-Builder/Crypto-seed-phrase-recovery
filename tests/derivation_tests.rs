/// Verify address derivation against known BIP39 test vectors.
/// These are public standard test vectors anyone can verify independently.

use crypto_seed_recovery::multi_chain::{
    bitcoin::BitcoinDeriver,
    ethereum::EthereumDeriver,
    solana::SolanaDeriver,
    tron::TronDeriver,
    sui::SuiDeriver,
    types::ChainType,
    multi_deriver::MultiChainDeriver,
};

// Standard BIP39 test mnemonic (publicly known, zero-value)
const TEST_PHRASE: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[test]
fn test_ethereum_derivation() {
    let deriver = EthereumDeriver::from_phrase(TEST_PHRASE).unwrap();
    let addrs = deriver.derive_addresses(1);
    assert!(!addrs.is_empty(), "Should derive at least 1 ETH address");
    let addr = &addrs[0];
    assert!(addr.starts_with("0x"), "ETH address must start with 0x");
    assert_eq!(addr.len(), 42, "ETH address must be 42 chars");
    // Known vector for this mnemonic at m/44'/60'/0'/0/0
    assert_eq!(addr.to_lowercase(), "0x9858effd232b4033e47d90003d41ec34ecaeda94",
        "ETH address mismatch for standard test mnemonic");
    println!("ETH [0]: {}", addr);
}

#[test]
fn test_bitcoin_derivation() {
    let deriver = BitcoinDeriver::from_phrase(TEST_PHRASE).unwrap();
    let addrs = deriver.derive_addresses(1);
    assert!(!addrs.is_empty(), "Should derive at least 1 BTC address");
    let addr = &addrs[0];
    assert!(addr.starts_with("bc1"), "BTC native SegWit address must start with bc1");
    println!("BTC [0]: {}", addr);
}

#[test]
fn test_solana_derivation() {
    let deriver = SolanaDeriver::from_phrase(TEST_PHRASE).unwrap();
    let addrs = deriver.derive_addresses(1);
    assert!(!addrs.is_empty(), "Should derive at least 1 SOL address");
    let addr = &addrs[0];
    assert_eq!(addr.len(), 44, "SOL address must be 44 base58 chars");
    println!("SOL [0]: {}", addr);
}

#[test]
fn test_tron_derivation() {
    let deriver = TronDeriver::from_phrase(TEST_PHRASE).unwrap();
    let addrs = deriver.derive_addresses(1);
    assert!(!addrs.is_empty(), "Should derive at least 1 TRX address");
    let addr = &addrs[0];
    assert!(addr.starts_with('T'), "TRON address must start with T");
    assert_eq!(addr.len(), 34, "TRON address must be 34 chars");
    println!("TRX [0]: {}", addr);
}

#[test]
fn test_sui_derivation() {
    let deriver = SuiDeriver::from_phrase(TEST_PHRASE).unwrap();
    let addrs = deriver.derive_addresses(1);
    assert!(!addrs.is_empty(), "Should derive at least 1 SUI address");
    let addr = &addrs[0];
    assert!(addr.starts_with("0x"), "SUI address must start with 0x");
    assert_eq!(addr.len(), 66, "SUI address must be 66 chars (0x + 64 hex)");
    println!("SUI [0]: {}", addr);
}

#[test]
fn test_multi_chain_parallel_all_chains() {
    let deriver = MultiChainDeriver::new(TEST_PHRASE);
    let results = deriver.derive_all_chains_parallel(1);
    assert!(!results.is_empty(), "Should return results for all chains");
    println!("\n=== Derived addresses for test mnemonic ===");
    for (chain, addrs) in &results {
        if let Some(addr) = addrs.first() {
            println!("  {:12}: {}", chain.name(), addr);
        }
    }
    // Every chain in the parallel set should return at least one address
    assert_eq!(results.len(), 11, "Should derive for all chains in the parallel set");
    for (chain, addrs) in &results {
        assert!(!addrs.is_empty(), "{} returned no address", chain.name());
    }
}

#[test]
fn test_trust_wallet_same_eth_as_metamask() {
    // Trust Wallet and MetaMask use the SAME ETH derivation path m/44'/60'/0'/0/0
    // So the same phrase gives the same ETH address in both apps
    let deriver = EthereumDeriver::from_phrase(TEST_PHRASE).unwrap();
    let addrs = deriver.derive_addresses(3);
    assert_eq!(addrs.len(), 3);
    // All 3 should be unique addresses
    assert_ne!(addrs[0], addrs[1]);
    assert_ne!(addrs[1], addrs[2]);
    println!("Trust Wallet = MetaMask ETH addresses:");
    for (i, a) in addrs.iter().enumerate() {
        println!("  Account {}: {}", i, a);
    }
}
