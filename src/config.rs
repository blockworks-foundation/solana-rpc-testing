use serde::{Deserialize, Serialize};
use solana_sdk::signature::Keypair;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProgramData {
    pub name: String,
    pub program_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub programs: Vec<ProgramData>,
    pub known_accounts: Vec<String>,
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
