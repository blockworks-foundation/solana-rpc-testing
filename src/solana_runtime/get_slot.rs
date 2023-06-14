use std::time::{Duration, Instant};

use log::info;

use crate::{
    bencher::{Bencher, Benchmark, Stats},
    cli::Args,
    config::Config,
    rpc_client::CustomRpcClient,
    test_registry::TestingTask,
};

#[derive(Clone)]
pub struct GetSlotTest;

#[async_trait::async_trait]
impl TestingTask for GetSlotTest {
    async fn test(&self, args: Args, _: Config) -> anyhow::Result<Stats> {
        let instant = GetSlotTest;
        let stats = Bencher::bench::<Self>(instant, args).await?;
        info!("{} {}", self.get_name(), serde_json::to_string(&stats)?);
        Ok(stats)
    }

    fn get_name(&self) -> String {
        "GetSlotTest".to_string()
    }
}

#[async_trait::async_trait]
impl Benchmark for GetSlotTest {
    async fn run(
        self,
        rpc_client: &mut CustomRpcClient,
        duration: Duration,
        _: u64,
    ) -> anyhow::Result<()> {
        let start = Instant::now();

        while start.elapsed() < duration {
            rpc_client.raw_get_slot().await;
        }

        Ok(())
    }
}
