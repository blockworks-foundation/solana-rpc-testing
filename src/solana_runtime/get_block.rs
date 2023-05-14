use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use log::info;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::slot_history::Slot;

use crate::{
    bencher::{Bencher, Benchmark, Run},
    cli::Args,
    config::Config,
    test_registry::TestingTask,
};

pub struct GetBlockTest;

#[async_trait::async_trait]
impl TestingTask for GetBlockTest {
    async fn test(&self, args: Args, _config: Config) -> anyhow::Result<()> {
        let metric = Bencher::bench::<GetBlockBench>(args).await?;
        info!("{}", serde_json::to_string(&metric)?);
        Ok(())
    }

    fn get_name(&self) -> String {
        "GetBlockTest".to_string()
    }
}

pub struct GetBlockBench {
    slot: Slot,
}

#[async_trait::async_trait]
impl Benchmark for GetBlockBench {
    async fn prepare(rpc_client: Arc<RpcClient>) -> anyhow::Result<Self> {
        Ok(Self {
            slot: rpc_client.get_slot().await?,
        })
    }

    async fn run(&mut self, rpc_client: Arc<RpcClient>, duration: Duration) -> anyhow::Result<Run> {
        let mut result = Run::default();

        let start = Instant::now();
        while start.elapsed() < duration {
            match rpc_client.get_block(self.slot).await {
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
