use crate::test_registry::TestingTask;
use async_trait::async_trait;
use const_env::from_env;
use dashmap::DashSet;
use rand::{distributions::Alphanumeric, prelude::Distribution, seq::SliceRandom, SeedableRng};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair,
    signer::Signer, transaction::Transaction,
};
use std::{
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

const MEMO_PROGRAM_ID: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

#[from_env]
const NB_MEMO_TRANSACTIONS_SENT_PER_SECOND: usize = 256;

#[derive(Clone, Debug, Copy)]
struct Metric {
    pub number_of_confirmed_txs: usize,
    pub number_of_unconfirmed_txs: usize,
}

pub struct SendAndConfrimTesting;

fn create_memo_tx(msg: &[u8], payer: &Keypair, blockhash: Hash) -> Transaction {
    let memo = Pubkey::from_str(MEMO_PROGRAM_ID).unwrap();

    let instruction = Instruction::new_with_bytes(memo, msg, vec![]);
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    Transaction::new(&[payer], message, blockhash)
}

fn generate_random_strings(num_of_txs: usize, random_seed: Option<u64>) -> Vec<Vec<u8>> {
    let seed = random_seed.map_or(0, |x| x);
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    (0..num_of_txs)
        .map(|_| Alphanumeric.sample_iter(&mut rng).take(10).collect())
        .collect()
}

async fn send_and_confirm_transactions(
    rpc_client: Arc<RpcClient>,
    tx_count: usize,
    funded_payer: Keypair,
    seed: u64,
    block_hash: Arc<RwLock<Hash>>,
) -> Metric {
    let map_of_txs = Arc::new(DashSet::new());
    // transaction sender task
    {
        let map_of_txs = map_of_txs.clone();
        let rpc_client = rpc_client.clone();
        tokio::spawn(async move {
            let map_of_txs = map_of_txs.clone();
            let rand_strings = generate_random_strings(tx_count, Some(seed));

            for rand_string in rand_strings {
                let blockhash = { *block_hash.read().await };
                let tx = create_memo_tx(&rand_string, &funded_payer, blockhash);
                if let Ok(signature) = rpc_client.send_transaction(&tx).await {
                    map_of_txs.insert(signature);
                }
            }
        });
    }

    let confirmation_time = Instant::now();
    let mut confirmed_count = 0;

    let mut metric = Metric {
        number_of_confirmed_txs: 0,
        number_of_unconfirmed_txs: 0,
    };

    while confirmation_time.elapsed() < Duration::from_secs(120)
        && !(map_of_txs.is_empty() && confirmed_count == tx_count)
    {
        let signatures = map_of_txs.iter().map(|x| *x.key()).collect::<Vec<_>>();
        if signatures.is_empty() {
            tokio::time::sleep(Duration::from_millis(1)).await;
            continue;
        }

        if let Ok(res) = rpc_client.get_signature_statuses(&signatures).await {
            for i in 0..signatures.len() {
                let tx_status = &res.value[i];
                if tx_status.is_some() {
                    let signature = signatures[i];
                    let tx_data = map_of_txs.get(&signature).unwrap();
                    metric.number_of_confirmed_txs += 1;
                    drop(tx_data);
                    map_of_txs.remove(&signature);
                    confirmed_count += 1;
                }
            }
        }
    }

    for _ in map_of_txs.iter() {
        metric.number_of_unconfirmed_txs += 1;
    }
    metric
}

#[async_trait]
impl TestingTask for SendAndConfrimTesting {
    async fn test(
        &self,
        args: crate::cli::Args,
        config: crate::config::Config,
    ) -> anyhow::Result<()> {
        if !args.test_send_and_confirm_transactions() {
            return Ok(());
        }

        println!("started sending and confirming memo transactions");
        let rpc_client = Arc::new(RpcClient::new(args.rpc_addr.clone()));
        let block_hash: Arc<RwLock<Hash>> = Arc::new(RwLock::new(Hash::default()));

        // block hash updater task
        {
            let block_hash = block_hash.clone();
            let rpc_client = rpc_client.clone();
            tokio::spawn(async move {
                loop {
                    let bh = rpc_client.get_latest_blockhash().await;
                    match bh {
                        Ok(bh) => {
                            let mut lock = block_hash.write().await;
                            *lock = bh;
                        }
                        Err(e) => println!("blockhash update error {}", e),
                    }
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });
        }

        let mut run_interval_ms = tokio::time::interval(Duration::from_secs(1));
        let nb_runs = args.duration_in_seconds;
        let mut tasks = vec![];
        let payers = config.get_payers();
        for seed in 0..nb_runs {
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed as u64);
            let payer = payers.choose(&mut rng).unwrap();
            let payer = Keypair::from_bytes(payer.to_bytes().as_slice()).unwrap();
            tasks.push(tokio::spawn(send_and_confirm_transactions(
                rpc_client.clone(),
                NB_MEMO_TRANSACTIONS_SENT_PER_SECOND,
                payer,
                seed as u64,
                block_hash.clone(),
            )));
            // wait for an interval
            run_interval_ms.tick().await;
        }

        let instant = Instant::now();
        let tasks_res = futures::future::join_all(tasks).await;
        let duration = instant.elapsed();
        let mut total_txs_confirmed = 0;
        let mut total_txs_unconfirmed = 0;

        for metric in tasks_res.into_iter().flatten() {
            total_txs_confirmed += metric.number_of_confirmed_txs;
            total_txs_unconfirmed += metric.number_of_unconfirmed_txs;
        }

        println!("Memo transaction sent and confrim results \n Number of transaction confirmed : {}, \n Number of transactions unconfirmed {}, took {}s", total_txs_confirmed, total_txs_unconfirmed, duration.as_secs());
        Ok(())
    }

    fn get_name(&self) -> String {
        "Send and confirm memo transaction".to_string()
    }
}
