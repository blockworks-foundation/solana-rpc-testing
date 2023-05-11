use std::sync::Arc;

use log::info;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

use crate::bencher::{BenchFn, Bencher};
use crate::{cli::Args, config::Config, test_registry::TestingTask};

pub struct GetSlotTest;

#[async_trait::async_trait]
impl TestingTask for GetSlotTest {
    async fn test(&self, args: Args, _config: Config) -> anyhow::Result<()> {
        let metric = Bencher::bench::<Self>(args).await?;
        info!("metric {}", serde_json::to_string(&metric)?);
        Ok(())
    }

    fn get_name(&self) -> &'static str {
        "GetSlotTest"
    }
}

#[async_trait::async_trait]
impl BenchFn for GetSlotTest {
    async fn new(_: Arc<RpcClient>) -> anyhow::Result<Self> {
        Ok(Self)
    }

    async fn bench_fn(&mut self, rpc_client: Arc<RpcClient>) -> anyhow::Result<()> {
        rpc_client.get_slot().await?;
        Ok(())
    }
}
