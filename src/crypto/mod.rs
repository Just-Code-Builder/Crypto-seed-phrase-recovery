//! Cryptographic primitives — published openly so anyone can audit that this
//! tool handles seed material correctly and never exfiltrates it.
//!
//! Note: there are NO network calls anywhere in this crate. Seed-handling and
//! key-derivation happen entirely in memory, locally.

pub mod fuzzy_match;
pub mod kdf;
pub mod seed_encryption;
