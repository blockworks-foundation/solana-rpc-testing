use std::sync::Arc;

use solana_rpc_client::nonblocking::rpc_client::RpcClient;

use crate::cli::Args;
use crate::metrics::{Metric, PartialMetric};

#[async_trait::async_trait]
pub trait BenchFn: Send + 'static {
    async fn new(rpc_client: Arc<RpcClient>) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn bench_fn(&mut self, rpc_client: Arc<RpcClient>) -> anyhow::Result<()>;
}

pub struct Bencher;

impl Bencher {
    pub async fn bench<B: BenchFn>(args: Args) -> anyhow::Result<Metric> {
        let rpc_client = args.get_rpc_client();

        let futs = (0..args.threads).map(|_| {
            let rpc_client = rpc_client.clone();
            let duration = args.get_duration_to_run_test();

            tokio::spawn(async move {
                let mut bench_fn = B::new(rpc_client.clone()).await.unwrap();

                let mut part_metric = PartialMetric::default();
                let thread_start = tokio::time::Instant::now();

                while thread_start.elapsed() <= duration {
                    let Err(err) = bench_fn.bench_fn(rpc_client.clone()).await else {
                        part_metric.passed += 1;
                        continue;
                    };

                    part_metric.failed += 1;
                    log::warn!("{err}");
                }

                part_metric.total_time = thread_start.elapsed();
                Metric::from(part_metric)
            })
        });

        let avg_metric = futures::future::try_join_all(futs)
            .await?
            .into_iter()
            .sum::<Metric>()
            / args.threads;

        Ok(avg_metric)
    }
}
