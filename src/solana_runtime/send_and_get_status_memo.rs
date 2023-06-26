use crate::{
    bencher::{Bencher, Benchmark, Stats},
    cli::Args,
    config::Config,
    rpc_client::CustomRpcClient,
    test_registry::TestingTask,
};
use async_trait::async_trait;
use rand::{distributions::Alphanumeric, prelude::Distribution, seq::SliceRandom, SeedableRng};
use solana_sdk::{
    hash::Hash, instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair,
    signer::Signer, transaction::Transaction,
};
use std::{str::FromStr, sync::Arc, time::Instant};
use tokio::sync::RwLock;

const MEMO_PROGRAM_ID: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

pub struct SendAndConfrimTesting {
    pub block_hash: Arc<RwLock<Hash>>,
}

fn create_memo_tx(msg: &[u8], payer: &Keypair, blockhash: Hash) -> Transaction {
    let memo = Pubkey::from_str(MEMO_PROGRAM_ID).unwrap();

    let instruction = Instruction::new_with_bytes(memo, msg, vec![]);
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    Transaction::new(&[payer], message, blockhash)
}

#[async_trait]
impl TestingTask for SendAndConfrimTesting {
    async fn test(&self, args: &Args, config: &Config) -> anyhow::Result<Stats> {
        let instant = SendMemoTransactionsBench {
            block_hash: self.block_hash.clone(),
            payers: Arc::new(config.users.iter().map(|x| x.get_keypair()).collect()),
        };
        let metric = Bencher::bench::<SendMemoTransactionsBench>(instant, args).await?;
        Ok(metric)
    }

    fn get_name(&self) -> &'static str {
        "Send and confirm memo transaction"
    }
}

#[derive(Clone)]
struct SendMemoTransactionsBench {
    block_hash: Arc<RwLock<Hash>>,
    payers: Arc<Vec<Keypair>>,
}

#[async_trait::async_trait]
impl Benchmark for SendMemoTransactionsBench {
    async fn run(
        self,
        rpc_client: &mut CustomRpcClient,
        duration: std::time::Duration,
        random_number: u64,
    ) -> anyhow::Result<()> {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(random_number);
        let start = Instant::now();
        while start.elapsed() < duration {
            let msg: Vec<u8> = Alphanumeric.sample_iter(&mut rng).take(10).collect();
            let payer = self.payers.choose(&mut rng).unwrap();

            let blockhash = { *self.block_hash.read().await };
            let tx = create_memo_tx(&msg, payer, blockhash);
            let _ = rpc_client.raw_send_transaction(tx).await;
        }

        Ok(())
    }
}
