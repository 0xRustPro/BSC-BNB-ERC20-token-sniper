use ethers::types::Address;
use std::{env, str::FromStr};


#[derive(Debug, Clone)]
pub struct Config {
    pub wss_url: String,
    pub factory_address: Address,
    pub private_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            wss_url: env::var("WSS_URL")
                .unwrap_or_else(|_| "wss://bsc-rpc.publicnode.com".into()),
            factory_address: Address::from_str(
                "0x79c7909097a2a5cedb8da900e3192cee671521a6",
            )
            .expect("Invalid factory address"),
            private_key: env::var("PRIVATE_KEY").ok(),
        }
    }
}
