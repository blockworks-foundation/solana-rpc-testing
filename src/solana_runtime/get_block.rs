use std::sync::Arc;

use log::info;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::slot_history::Slot;

use crate::{
    bencher::{BenchFn, Bencher},
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

    fn get_name(&self) -> &'static str {
        "GetBlockTest"
    }
}

pub struct GetBlockBench {
    slot: Slot,
}

#[async_trait::async_trait]
impl BenchFn for GetBlockBench {
    async fn new(rpc_client: Arc<RpcClient>) -> anyhow::Result<Self> {
        Ok(Self {
            slot: rpc_client.get_slot().await?,
        })
    }

    async fn bench_fn(&mut self, rpc_client: Arc<RpcClient>) -> anyhow::Result<()> {
      //  self.slot += 1;
        rpc_client.get_block(self.slot).await?;
        Ok(())
    }
}
