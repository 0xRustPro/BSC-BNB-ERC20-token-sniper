use ethers::types::{Address, H256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenEvent {
    pub contract: Address,
    pub ticker: String,
    pub creator: Option<Address>,
    pub tx_hash: Option<H256>,
    pub buy_amount: f64
}

#[derive(Debug, Clone)]
pub enum BotEvent {
    TokenCreated(TokenEvent),
    Shutdown,
}
