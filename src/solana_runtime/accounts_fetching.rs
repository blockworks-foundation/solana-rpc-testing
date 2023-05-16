use std::{
    str::FromStr,
    sync::{Arc},
};
use async_trait::async_trait;
use const_env::from_env;
use rand::{seq::IteratorRandom, SeedableRng};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use tokio::time::Instant;

use crate::{config::Config, test_registry::TestingTask, bencher::{Bencher, Benchmark, Run}};

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
    async fn test(&self, args: crate::cli::Args, config: Config) -> anyhow::Result<()> {
        let accounts = config
            .known_accounts
            .iter()
            .map(|x| Pubkey::from_str(x.as_str()).unwrap())
            .collect::<Vec<_>>();
        let unknown_accounts: Vec<Pubkey> =
            AccountsFetchingTests::create_random_address(accounts.len());

        let instant = GetAccountsBench { 
            accounts_list: [accounts, unknown_accounts].concat(),
        };
        let metric = Bencher::bench::<GetAccountsBench>(instant, args).await?;
        log::info!("{} {}", self.get_name(), serde_json::to_string(&metric)?);
        Ok(())
    }

    fn get_name(&self) -> String {
        "Accounts Fetching".to_string()
    }
}

#[derive(Clone)]
pub struct GetAccountsBench {
    accounts_list: Vec<Pubkey>,
}

#[async_trait::async_trait]
impl Benchmark for GetAccountsBench {

    async fn run(self, rpc_client: Arc<RpcClient>, duration: std::time::Duration, random_number: u64) -> anyhow::Result<crate::bencher::Run> {
        let mut result = Run::default();
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(random_number);
        let number_of_fetched_accounts = NB_OF_ACCOUNTS_FETCHED_PER_TASK.min(self.accounts_list.len());
        let start = Instant::now();
        while start.elapsed() < duration {
            let accounts = self.accounts_list
                        .iter()
                        .copied()
                        .choose_multiple(&mut rng, number_of_fetched_accounts);

            match rpc_client
            .get_multiple_accounts(accounts.as_slice())
            .await {
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
