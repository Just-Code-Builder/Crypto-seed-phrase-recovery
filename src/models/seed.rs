use serde::{Deserialize, Serialize};

use crate::multi_chain::types::ChainType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecoveryMode {
    Valid,
    MissingWords,
    MisspelledWords,
    Combined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeedLength {
    Twelve,
    TwentyFour,
}

impl SeedLength {
    pub fn word_count(&self) -> usize {
        match self {
            Self::Twelve => 12,
            Self::TwentyFour => 24,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedAnalysis {
    pub word_count: usize,
    pub mode: RecoveryMode,
    pub missing_positions: Vec<usize>,
    pub misspelled: Vec<(usize, String, String, f32)>,
    pub candidate_count: u64,
    pub estimated_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryResult {
    pub seed_phrase: String,
    pub confidence: f32,
    pub chain: ChainType,
    pub address: String,
    pub corrections: Vec<String>,
}
