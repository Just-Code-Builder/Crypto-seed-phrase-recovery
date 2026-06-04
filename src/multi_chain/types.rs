use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    // EVM
    Ethereum,
    Polygon,
    Arbitrum,
    Optimism,
    Base,
    BnbSmartChain,
    Avalanche,
    // Non-EVM
    Bitcoin,
    Solana,
    Sui,
    Ton,
    Tron,
    Xrp,
}

impl ChainType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ethereum     => "Ethereum",
            Self::Polygon      => "Polygon",
            Self::Arbitrum     => "Arbitrum",
            Self::Optimism     => "Optimism",
            Self::Base         => "Base",
            Self::BnbSmartChain => "BNB Smart Chain",
            Self::Avalanche    => "Avalanche",
            Self::Bitcoin      => "Bitcoin",
            Self::Solana       => "Solana",
            Self::Sui          => "SUI",
            Self::Ton          => "TON",
            Self::Tron         => "TRON",
            Self::Xrp          => "XRP",
        }
    }

    pub fn derivation_path(&self, index: u32) -> String {
        match self {
            Self::Bitcoin      => format!("m/44'/0'/0'/0/{}", index),
            Self::Solana       => format!("m/44'/501'/{}'/0'", index),
            Self::Sui          => format!("m/44'/784'/{}'/0'/0'", index),
            Self::Ton          => "m/44'/607'/0'".to_string(),
            Self::Tron         => format!("m/44'/195'/0'/0/{}", index),
            Self::Xrp          => format!("m/44'/144'/0'/0/{}", index),
            _                  => format!("m/44'/60'/0'/0/{}", index),
        }
    }

    pub fn is_evm(&self) -> bool {
        matches!(self,
            Self::Ethereum | Self::Polygon | Self::Arbitrum |
            Self::Optimism | Self::Base | Self::BnbSmartChain | Self::Avalanche
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub chain: ChainType,
    pub address: String,
    pub derivation_index: usize,
}
