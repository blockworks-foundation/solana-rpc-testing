use serde::{Deserialize, Serialize};
use solana_sdk::{signature::Keypair, signer::Signer};

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
    pub name: String,
    pub instruction: Vec<u8>,
    pub argument_sizes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProgramData {
    pub name: String,
    pub program_id: String,
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub secret: Vec<u8>,
    pub token_data: Vec<TokenAccountData>,
    pub open_orders: Vec<OpenOrders>,
}

impl User {
    pub fn get_keypair(&self) -> Keypair {
        Keypair::from_bytes(&self.secret).unwrap()
    }
}

impl Signer for User {
    fn try_pubkey(&self) -> Result<solana_sdk::pubkey::Pubkey, solana_sdk::signer::SignerError> {
        let kp = self.get_keypair();
        Ok(kp.pubkey())
    }

    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        let kp = self.get_keypair();
        Ok(kp.sign_message(message))
    }

    fn is_interactive(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenAccountData {
    pub mint: String,
    pub token_account: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenOrders {
    pub market: String,
    pub open_orders: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Market {
    pub name: String,
    pub admin: Vec<u8>,
    pub market_pk: String,
    pub oracle: String,
    pub asks: String,
    pub bids: String,
    pub event_queue: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub market_index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub programs: Vec<ProgramData>,
    pub known_accounts: Vec<String>,
    pub users: Vec<User>,
    pub mints: Vec<String>,
    pub markets: Vec<Market>,
}
