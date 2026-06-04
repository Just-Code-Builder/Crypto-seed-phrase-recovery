use rayon::prelude::*;

use super::{
    bitcoin::BitcoinDeriver,
    ethereum::EthereumDeriver,
    solana::SolanaDeriver,
    sui::SuiDeriver,
    ton::TonDeriver,
    tron::TronDeriver,
    xrp::XrpDeriver,
    types::ChainType,
};

pub struct MultiChainDeriver {
    phrase: String,
}

impl MultiChainDeriver {
    pub fn new(phrase: &str) -> Self {
        Self { phrase: phrase.to_string() }
    }

    pub fn derive_addresses(&self, chain: ChainType, count: usize) -> Vec<String> {
        match chain {
            ChainType::Bitcoin => BitcoinDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),

            ChainType::Ethereum
            | ChainType::Polygon
            | ChainType::Arbitrum
            | ChainType::Optimism
            | ChainType::Base
            | ChainType::BnbSmartChain
            | ChainType::Avalanche => EthereumDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),

            ChainType::Solana => SolanaDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),

            ChainType::Sui => SuiDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),

            ChainType::Ton => TonDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),

            ChainType::Tron => TronDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),

            ChainType::Xrp => XrpDeriver::from_phrase(&self.phrase)
                .map(|d| d.derive_addresses(count))
                .unwrap_or_default(),
        }
    }

    pub fn derive_all_chains_parallel(&self, count: usize) -> Vec<(ChainType, Vec<String>)> {
        let chains = [
            ChainType::Bitcoin,
            ChainType::Ethereum,
            ChainType::Solana,
            ChainType::Polygon,
            ChainType::Arbitrum,
            ChainType::Optimism,
            ChainType::Base,
            ChainType::Sui,
            ChainType::Ton,
            ChainType::Tron,
            ChainType::Xrp,
        ];

        chains
            .par_iter()
            .map(|chain| (*chain, self.derive_addresses(*chain, count)))
            .collect()
    }
}
