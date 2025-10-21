use ethers::types::{H256, Log, H160};
use ethers::utils::keccak256;
use eyre::Result;
use ethabi::{ParamType, decode};

use crate::listener::TokenCreated;

pub fn keccak256_hex(sig: &str) -> [u8; 32] {
    keccak256(sig)
}

pub fn parse_token_created_log(log: &Log) -> Result<TokenCreated> {
    let creator = H160::from_slice(&log.topics[1].as_bytes()[12..]);
    let token = H160::from_slice(&log.topics[2].as_bytes()[12..]);

    let decoded = decode(
        &[ParamType::String, ParamType::String],
        &log.data.0
    )?;

    let name = decoded[0].clone().into_string().unwrap_or_default();
    let symbol = decoded[1].clone().into_string().unwrap_or_default();

    Ok(TokenCreated { creator, token, name, symbol })
}
