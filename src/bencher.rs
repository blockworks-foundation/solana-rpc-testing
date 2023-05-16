use crate::cli::Args;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use solana_program::hash::Hash;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub type BlockHashGetter = Arc<RwLock<Hash>>;

#[async_trait::async_trait]
pub trait Benchmark: Clone + Send + 'static {
    async fn run(
        self,
        rpc_client: Arc<RpcClient>,
        duration: Duration,
        random_number: u64,
    ) -> anyhow::Result<Run>;
}

#[derive(Default, Serialize)]
pub struct Run {
    pub requests_completed: u64,
    pub requests_failed: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub errors: Vec<String>,
}

#[derive(Default, Serialize, Clone)]
pub struct Stats {
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub time_per_request: f64,
    pub total_transferred: u64,
    pub top_5_errors: Vec<(String, usize)>,
    pub average_number_of_requests_by_a_task: f64,
    pub total_requests_succeded: u64,
    pub total_requests_failed: u64,
    pub average_succeds_per_task: f64,
    pub average_failed_per_task: f64,
}

pub struct Bencher;

impl Bencher {
    pub async fn bench<B: Benchmark + Send + Clone>(
        instant: B,
        args: Args,
    ) -> anyhow::Result<Stats> {
        let start = Instant::now();
        let mut random = StdRng::seed_from_u64(0);
        let futs = (0..args.threads).map(|_| {
            let rpc_client = args.get_rpc_client();
            let duration = args.get_duration_to_run_test();
            let random_number = random.gen();
            let instance = instant.clone();
            tokio::spawn(async move {
                instance
                    .run(rpc_client.clone(), duration, random_number)
                    .await
                    .unwrap()
            })
        });

        let all_results = futures::future::try_join_all(futs).await?;

        let time = start.elapsed();

        let total_requests = all_results
            .iter()
            .fold(0, |acc, r| acc + r.requests_completed + r.requests_failed);

        let total_requests_succeded = all_results
            .iter()
            .fold(0, |acc, r| acc + r.requests_completed);
        let total_requests_failed = all_results.iter().fold(0, |acc, r| acc + r.requests_failed);

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
            time_per_request: time.as_secs_f64() / (total_requests as f64 / args.threads as f64),
            total_transferred,
            top_5_errors,
            average_number_of_requests_by_a_task: (total_requests as f64) / (args.threads as f64),
            total_requests_succeded,
            total_requests_failed,
            average_succeds_per_task: total_requests_succeded as f64 / args.threads as f64,
            average_failed_per_task: total_requests_failed as f64 / args.threads as f64,
        })
    }
}
