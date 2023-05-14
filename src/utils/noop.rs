use chrono::Utc;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use std::str::FromStr;

pub fn instruction(data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: Pubkey::from_str("zSZRvv8VgXtKNyw9t8fu4QKC4TL2P9DfSsvzBmBL8yn").unwrap(),
        accounts: vec![],
        data,
    }
}

pub fn timestamp() -> Instruction {
    instruction(Utc::now().timestamp_micros().to_le_bytes().into())
}
