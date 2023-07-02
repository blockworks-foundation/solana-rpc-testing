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
    pub fn create_random_address(count: usize) -> Vec<(Pubkey, u32)> {
        (0..count).map(|_| (Keypair::new().pubkey(), 0)).collect()
    }
}

#[async_trait]
impl TestingTask for AccountsFetchingTests {
    async fn test(&self, args: &Args, config: &Config) -> anyhow::Result<Stats> {
        let accounts = config
            .known_accounts
            .iter()
            .map(|x| (Pubkey::from_str(&x.0).unwrap(), x.1))
            .collect::<Vec<_>>();

        let unknown_accounts = AccountsFetchingTests::create_random_address(accounts.len());

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
    accounts_list: Arc<Vec<(Pubkey, u32)>>,
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
                .choose_multiple(&mut rng, number_of_fetched_accounts);

            // filter accounts whose account.1 value summing up to 10485760
            let mut sum = 0;
            let mut filtered_accounts = Vec::new();

            for account in accounts.iter() {
                let local_sum = sum + account.1;

                // try maximise the number of accounts fetched
                if local_sum <= 10485760 {
                    sum = local_sum;
                    filtered_accounts.push(account.0);
                } 
            }

            rpc_client
                .raw_get_multiple_accounts(filtered_accounts)
                .await
        }

        Ok(())
    }
}
