use std::sync::Arc;
use std::time::{Duration, Instant};

use log::info;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

use crate::bencher::{Bencher, Benchmark, Run, Stats};
use crate::{cli::Args, config::Config, test_registry::TestingTask};

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
        rpc_client: Arc<RpcClient>,
        duration: Duration,
        _: u64,
    ) -> anyhow::Result<Run> {
        let mut result = Run::default();

        let start = Instant::now();
        while start.elapsed() < duration {
            match rpc_client.get_slot().await {
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
