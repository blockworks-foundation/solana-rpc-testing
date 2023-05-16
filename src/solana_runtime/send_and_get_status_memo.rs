use crate::{test_registry::TestingTask, bencher::{Benchmark, Run, Bencher}, config::Config};
use async_trait::async_trait;
use rand::{distributions::Alphanumeric, prelude::Distribution, seq::SliceRandom, SeedableRng};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair,
    signer::Signer, transaction::Transaction,
};
use std::{
    str::FromStr,
    sync::Arc,
    time::{Instant},
};
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
    async fn test(
        &self,
        args: crate::cli::Args,
        config: crate::config::Config,
    ) -> anyhow::Result<()> {
        let instant = SendMemoTransactionsBench {
            block_hash: self.block_hash.clone(),
        };
        let metric = Bencher::bench::<SendMemoTransactionsBench>( instant, args, config).await?;
        log::info!("{} {}", self.get_name(), serde_json::to_string(&metric)?);
        Ok(())
    }

    fn get_name(&self) -> String {
        "Send and confirm memo transaction".to_string()
    }
}

#[derive(Clone)]
struct SendMemoTransactionsBench {
    block_hash:  Arc<RwLock<Hash>>,
}

#[async_trait::async_trait]
impl Benchmark for SendMemoTransactionsBench {

    async fn run(self, rpc_client: Arc<RpcClient>, duration: std::time::Duration, _: crate::cli::Args, config: Config, random_number: u64) -> anyhow::Result<crate::bencher::Run> {
        let mut result = Run::default();

        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(random_number);
        let start = Instant::now();
        while start.elapsed() < duration {
            let msg: Vec<u8> = Alphanumeric.sample_iter(&mut rng).take(10).collect();
            let payer = config.users.choose(&mut rng).unwrap();

            let blockhash = { *self.block_hash.read().await };
                let tx = create_memo_tx(&msg, &payer.get_keypair(), blockhash);
                match rpc_client.send_transaction(&tx).await {
                Ok(_) => {
                    result.requests_completed += 1;
                    result.bytes_received += 0;
                }
                Err(e) => {
                    result.requests_failed += 1;
                    result.errors.push(format!("{:?}", e.kind()));
                }
            }
            result.bytes_sent += 0;
        }
        Ok(result)
    }
}
