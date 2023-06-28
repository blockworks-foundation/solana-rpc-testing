use async_trait::async_trait;
use const_env::from_env;
use rand::{seq::IteratorRandom, SeedableRng};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::{str::FromStr, sync::Arc};
use tokio::time::Instant;

use crate::{
    bencher::{Bencher, Benchmark, Stats},
    cli::Args,
    config::Config,
    rpc_client::CustomRpcClient,
    test_registry::TestingTask,
};

#[from_env]
const NB_OF_ACCOUNTS_FETCHED_PER_TASK: usize = 100;

pub struct AccountsFetchingTests;

impl AccountsFetchingTests {
    pub fn create_random_address(count: usize) -> Vec<Pubkey> {
        (0..count).map(|_| Keypair::new().pubkey()).collect()
    }
}

#[async_trait]
impl TestingTask for AccountsFetchingTests {
    async fn test(&self, args: &Args, config: &Config) -> anyhow::Result<Stats> {
        let accounts = config
            .known_accounts
            .iter()
            .map(|x| Pubkey::from_str(x.as_str()).unwrap())
            .collect::<Vec<_>>();
        let unknown_accounts: Vec<Pubkey> =
            AccountsFetchingTests::create_random_address(accounts.len());

        let instant = GetAccountsBench {
            accounts_list: Arc::new([accounts, unknown_accounts].concat()),
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
    accounts_list: Arc<Vec<Pubkey>>,
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
        let number_of_fetched_accounts =
            NB_OF_ACCOUNTS_FETCHED_PER_TASK.min(self.accounts_list.len());
        let start = Instant::now();

        while start.elapsed() < duration {
            let accounts = self
                .accounts_list
                .iter()
                .copied()
                .choose_multiple(&mut rng, number_of_fetched_accounts);

            rpc_client.raw_get_multiple_accounts(accounts).await
        }

        Ok(())
    }
}
