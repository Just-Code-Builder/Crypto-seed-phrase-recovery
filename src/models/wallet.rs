use serde::{Deserialize, Serialize};

use crate::multi_chain::types::ChainType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub chain: ChainType,
    pub addresses: Vec<WalletAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddress {
    pub address: String,
    pub derivation_path: String,
    pub index: u32,
    pub balance: Option<Balance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub native_amount: f64,
    pub native_symbol: String,
    pub usd_value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBook {
    pub bitcoin: Vec<String>,
    pub ethereum: Vec<String>,
    pub solana: Vec<String>,
    pub polygon: Vec<String>,
    pub arbitrum: Vec<String>,
    pub optimism: Vec<String>,
    pub base: Vec<String>,
}
