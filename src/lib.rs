//! # Crypto Seed Recovery — Audit Library
//!
//! This is the **public, auditable** portion of Crypto Seed Recovery.
//!
//! It contains the parts that matter for trust and safety:
//!   - [`multi_chain`] — HD address derivation for 13 chains (BIP-32 / SLIP-0010).
//!     All standard, all verifiable against known test vectors (`cargo test`).
//!   - [`crypto`] — AES-256-GCM seed encryption, Argon2 KDF, Levenshtein fuzzy
//!     matching. Proves seed material is handled in-memory and never sent anywhere.
//!   - [`models`] — the data types used for seeds and wallets.
//!
//! The brute-force recovery engine, the on-chain auto-split payment logic, and
//! the developer payment addresses live in a separate private codebase and ship
//! only inside the official signed binaries.
//!
//! Official downloads: <https://github.com/Just-Code-Builder/Crypto-seed-phrase-recovery/releases>

pub mod crypto;
pub mod error;
pub mod models;
pub mod multi_chain;
