use async_trait::async_trait;
use rand::{seq::IteratorRandom, SeedableRng};
use std::sync::Arc;
use tokio::time::Instant;

use crate::{
    bencher::{Bencher, Benchmark, Stats},
    cli::Args,
    config::Config,
    rpc_client::CustomRpcClient,
    test_registry::TestingTask,
};

pub struct AccountsFetchingTests;

#[async_trait]
impl TestingTask for AccountsFetchingTests {
    async fn test(&self, args: &Args, config: &Config) -> anyhow::Result<Stats> {
        let instant = GetAccountsBench {
            accounts_list: Arc::new(config.known_accounts.clone()),
        };

        let metric = Bencher::bench::<GetAccountsBench>(instant, args).await?;
        Ok(metric)
    }

    fn get_name(&self) -> String {
        "Accounts Fetching".to_string()
    }
}

#[derive(Clone)]
pub struct GetAccountsBench {
    accounts_list: Arc<Vec<String>>,
}

#[async_trait::async_trait]
impl Benchmark for GetAccountsBench {
    async fn run(
        self,
        rpc_client: &mut CustomRpcClient,
        duration: std::time::Duration,
        random_number: u64,
    ) -> anyhow::Result<()> {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(random_number);

        let start = Instant::now();

        while start.elapsed() < duration {
            // get single random account from accounts_list
            let account = self.accounts_list.iter().choose(&mut rng).unwrap();

            rpc_client.raw_get_account_info(account).await
        }

        Ok(())
    }
}
