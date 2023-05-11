use serde::{Deserialize, Serialize};
use solana_sdk::signature::Keypair;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProgramData {
    pub name: String,
    pub program_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    secret: Vec<u8>,
    token_data: Vec<TokenAccountData>,
    open_orders: Vec<OpenOrders>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenAccountData {
    mint: String,
    token_account: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenOrders {
    market: String,
    open_orders: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Market {
    name: String,
    admin : Vec<u8>,
    market_pk: String,
    oracle: String,
    asks: String,
    bids: String,
    event_queue: String,
    base_vault: String,
    quote_vault: String,
    base_mint: String,
    quote_mint: String,
    market_index: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub programs: Vec<ProgramData>,
    pub known_accounts: Vec<String>,
    pub users: Vec<User>,
    pub mints: Vec<String>,
    pub markets: Vec<Market>,
}

impl Config {
    pub fn get_payers(&self) -> Vec<Keypair> {
        self.users
            .iter()
            .map(|x| Keypair::from_bytes(x.secret.as_slice()).unwrap())
            .collect()
    }
}
