pub mod bencher;
mod cli;
mod config;
mod openbook;
mod solana_runtime;
mod test_registry;
mod utils;

use std::{sync::Arc, time::Duration};

use anyhow::{bail, Context};
use clap::Parser;
use cli::Args;
use config::Config;
use solana_sdk::hash::Hash;
use tokio::sync::RwLock;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    assert_ne!(args.threads, 0, "Threads can't be 0");
    assert_ne!(args.duration_in_seconds, 0, "Duration can't be 0");

    let contents = std::fs::read_to_string(&args.config_file)
        .context("Should have been able to read the file")?;
    let config_json: Config = serde_json::from_str(&contents).context("Config file not valid")?;

    if config_json.users.is_empty() {
        log::error!("Config file is missing payers");
        bail!("No payers");
    }

    if config_json.markets.is_empty() {
        log::error!("Config file is missing markets");
        bail!("No markets")
    }

    let rpc_client = args.get_rpc_client();
    let current_hash = rpc_client.get_latest_blockhash().await.unwrap();
    let block_hash: Arc<RwLock<Hash>> = Arc::new(RwLock::new(current_hash));
    // block hash updater task
    {
        let block_hash = block_hash.clone();
        let rpc_client = args.get_rpc_client();
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

    args.generate_test_registry(block_hash)
        .start_testing(args, config_json)
        .await;

    Ok(())
}
