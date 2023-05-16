use std::sync::Arc;
use std::time::{Duration, Instant};
use itertools::Itertools;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use serde::Serialize;
use solana_program::hash::Hash;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use tokio::sync::RwLock;
use crate::cli::Args;

pub type BlockHashGetter = Arc<RwLock<Hash>>;

#[async_trait::async_trait]
pub trait Benchmark: Clone + Send + 'static {
    async fn run(self, rpc_client: Arc<RpcClient>, duration: Duration, random_number: u64) -> anyhow::Result<Run>;
}

#[derive(Default, Serialize)]
pub struct Run {
    pub requests_completed: u64,
    pub requests_failed: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub errors: Vec<String>,
}

#[derive(Default, Serialize)]
pub struct Stats {
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub time_per_request: f64,
    pub total_transferred: u64,
    pub top_5_errors: Vec<(String, usize)>,
    #[serde(flatten)]
    pub all_runs: Vec<Run>,
}

pub struct Bencher;

impl Bencher {
    pub async fn bench<B: Benchmark + Send + Clone>(instant: B, args: Args) -> anyhow::Result<Stats> {
        let start = Instant::now();
        let mut random = StdRng::seed_from_u64(0);
        let futs = (0..args.threads).map(|_| {
            let rpc_client = args.get_rpc_client();
            let duration = args.get_duration_to_run_test();
            let random_number = random.gen();
            let instant = instant.clone();
            tokio::spawn(async move {
                instant.run(rpc_client.clone(), duration, random_number).await.unwrap()
            })
        });

        let all_results = futures::future::try_join_all(futs).await?;

        let time = start.elapsed();

        let total_requests = all_results
            .iter()
            .fold(0, |acc, r| acc + r.requests_completed + r.requests_failed);
        let total_transferred = all_results
            .iter()
            .fold(0, |acc, r| acc + r.bytes_sent + r.bytes_received);
        let all_errors = all_results.iter().flat_map(|r| &r.errors).counts();
        let top_5_errors = all_errors
            .iter()
            .sorted_by_key(|(_e, c)| *c)
            .rev()
            .take(5)
            .map(|(e, c)| ((*e).clone(), c.clone()))
            .collect_vec();

        Ok(Stats {
            total_requests,
            requests_per_second: total_requests as f64 / time.as_secs_f64(),
            time_per_request: time.as_secs_f64() / total_requests as f64,
            total_transferred,
            top_5_errors,
            all_runs: all_results,
        })
    }
}