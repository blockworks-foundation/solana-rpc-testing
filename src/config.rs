use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub known_accounts: Vec<Pubkey>,
    pub payers: Vec<Vec<u8>>,
}

impl Config {
    pub fn get_payers(&self) -> Vec<Keypair> {
        self.payers
            .iter()
            .map(|x| Keypair::from_bytes(x.as_slice()).unwrap())
            .collect()
    }
}
