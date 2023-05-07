use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use const_env::from_env;
use rand::{seq::IteratorRandom, SeedableRng};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use tokio::time::Instant;

use crate::{config::Config, test_registry::TestingTask};

#[from_env]
const NB_ACCOUNT_FETCHING_TASKS: usize = 10;

#[from_env]
const NB_OF_ACCOUNTS_FETCHED_PER_TASK: usize = 256;

pub struct AccountsFetchingTests;

impl AccountsFetchingTests {
    pub fn create_random_address(count: usize) -> Vec<Pubkey> {
        (0..count).map(|_| Keypair::new().pubkey()).collect()
    }
}

#[async_trait]
impl TestingTask for AccountsFetchingTests {
    async fn test(&self, args: crate::cli::Args, config: Config) -> anyhow::Result<()> {
        if !args.test_accounts_fetching() {
            return Ok(());
        }
        let rpc_client = Arc::new(RpcClient::new(args.rpc_addr.clone()));

        let known_accounts = config
            .known_accounts
            .iter()
            .map(|x| Pubkey::from_str(x.as_str()).unwrap())
            .collect::<Vec<_>>();
        let unknown_accounts: Vec<Pubkey> =
            AccountsFetchingTests::create_random_address(known_accounts.len());
        let number_of_fetched_accounts =
            NB_OF_ACCOUNTS_FETCHED_PER_TASK.max(unknown_accounts.len());

        for i in 0..NB_ACCOUNT_FETCHING_TASKS {
            // each new task will fetch (100/NB_ACCOUNT_FETCHING_TASKS) * i percent of unknown accounts
            // so first task will fetch no unknown accounts and last will fetch almost all unknown accounts
            let known_accounts = known_accounts.clone();
            let unknown_accounts = unknown_accounts.clone();
            let duration = args.get_duration_to_run_test();
            let rpc_client = rpc_client.clone();
            tokio::spawn(async move {
                let percentage_of_unknown_tasks =
                    100.0 / (NB_ACCOUNT_FETCHING_TASKS as f64) * (i as f64);
                let unknown_accounts_to_take =
                    ((number_of_fetched_accounts as f64) * percentage_of_unknown_tasks) as usize;
                let known_accounts_to_take = number_of_fetched_accounts - unknown_accounts_to_take;
                let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(i as u64);

                let instant = Instant::now();
                while instant.elapsed() < duration {
                    let known_accounts = known_accounts
                        .iter()
                        .choose_multiple(&mut rng, known_accounts_to_take);
                    let unknown_accounts = unknown_accounts
                        .iter()
                        .choose_multiple(&mut rng, unknown_accounts_to_take);
                    let accounts_to_fetch = [known_accounts, unknown_accounts]
                        .concat()
                        .iter()
                        .map(|x| (*x).clone())
                        .collect::<Vec<_>>();
                    let _ = rpc_client
                        .get_multiple_accounts(accounts_to_fetch.as_slice())
                        .await;
                }
            });
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "Accounts Fetching".to_string()
    }
}
