use std::time::{Duration, Instant};

use log::info;
use solana_sdk::slot_history::Slot;

use crate::{
    bencher::{Bencher, Benchmark, Stats},
    cli::Args,
    config::Config,
    rpc_client::CustomRpcClient,
    test_registry::TestingTask,
};

pub struct GetBlockTest;

#[async_trait::async_trait]
impl TestingTask for GetBlockTest {
    async fn test(&self, args: Args, _: Config) -> anyhow::Result<Stats> {
        let slot = args.get_rpc_client().get_slot().await.unwrap();
        let instant = GetBlockBench { slot };
        let metric = Bencher::bench::<GetBlockBench>(instant, args).await?;
        info!("{} {}", self.get_name(), serde_json::to_string(&metric)?);
        Ok(metric)
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
    async fn run(
        self,
        rpc_client: &mut CustomRpcClient,
        duration: Duration,
        _: u64,
    ) -> anyhow::Result<()> {
        let start = Instant::now();

        while start.elapsed() < duration {
            rpc_client.raw_get_block(self.slot).await;
        }

        Ok(())
    }
}
