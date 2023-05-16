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
    async fn test(&self, args: Args, config: Config) -> anyhow::Result<()> {
        let slot =  {
            args.get_rpc_client().get_slot().await.unwrap()
        };
        let instant = GetBlockBench {
            slot
        };
        let metric = Bencher::bench::<GetBlockBench>( instant, args, config).await?;
        info!("{}", serde_json::to_string(&metric)?);
        Ok(())
    }

    fn get_name(&self) -> String {
        "GetBlockTest".to_string()
    }
}

#[derive(Clone)]
pub struct GetBlockBench {
    slot: Slot,
}

#[async_trait::async_trait]
impl Benchmark for GetBlockBench {

    async fn run(self, rpc_client: Arc<RpcClient>, duration: Duration, _: Args, _: Config, _: u64) -> anyhow::Result<Run> {
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
