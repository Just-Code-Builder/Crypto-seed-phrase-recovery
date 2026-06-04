use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("Invalid seed length: expected 12 or 24 words, got {0}")]
    InvalidSeedLength(usize),

    #[error("Too many missing words: max {max}, got {count}")]
    TooManyMissingWords { max: usize, count: usize },

    #[error("Invalid BIP39 word at position {position}: '{word}'")]
    InvalidWord { position: usize, word: String },

    #[error("No matching seed found after testing {tested} candidates")]
    NoMatchFound { tested: u64 },

    #[error("Invalid wallet address: {0}")]
    InvalidAddress(String),

    #[error("Chain not supported: {0}")]
    UnsupportedChain(String),

    #[error("Derivation error: {0}")]
    DerivationError(String),

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Service unavailable: {0}")]
    Unavailable(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Parse error: {0}")]
    Parse(String),
}
