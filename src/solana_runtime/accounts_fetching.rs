use std::{
    collections::HashSet,
    str::FromStr,
    sync::{atomic::AtomicU64, Arc},
};

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
        let rpc_client = Arc::new(RpcClient::new(args.rpc_addr.clone()));
        let total_fetches = Arc::new(AtomicU64::new(0));

        let known_accounts = config
            .known_accounts
            .iter()
            .map(|x| Pubkey::from_str(x.as_str()).unwrap())
            .collect::<Vec<_>>();
        let unknown_accounts: Vec<Pubkey> =
            AccountsFetchingTests::create_random_address(known_accounts.len());
        let number_of_fetched_accounts = NB_OF_ACCOUNTS_FETCHED_PER_TASK.min(known_accounts.len());

        let hash_set_known = Arc::new(known_accounts.iter().copied().collect::<HashSet<_>>());

        let mut tasks = vec![];
        for i in 0..NB_ACCOUNT_FETCHING_TASKS {
            // each new task will fetch (100/NB_ACCOUNT_FETCHING_TASKS) * i percent of unknown accounts
            // so first task will fetch no unknown accounts and last will fetch almost all unknown accounts
            let known_accounts = known_accounts.clone();
            let unknown_accounts = unknown_accounts.clone();
            let duration = args.get_duration_to_run_test();
            let rpc_client = rpc_client.clone();
            let hash_set_known = hash_set_known.clone();
            let total_fetches = total_fetches.clone();
            let task = tokio::spawn(async move {
                let percentage_of_unknown_tasks =
                    (100.0 / (NB_ACCOUNT_FETCHING_TASKS as f64)) * (i as f64);

                println!("started fetching accounts task #{}", i);
                let unknown_accounts_to_take = ((number_of_fetched_accounts as f64)
                    * percentage_of_unknown_tasks
                    / 100.0) as usize;
                let known_accounts_to_take =
                    number_of_fetched_accounts.saturating_sub(unknown_accounts_to_take);

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
                        .map(|x| *(*x))
                        .collect::<Vec<_>>();
                    let res = rpc_client
                        .get_multiple_accounts(accounts_to_fetch.as_slice())
                        .await;
                    total_fetches.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    match res {
                        Ok(res) => {
                            if res.len() == accounts_to_fetch.len() {
                                for i in 0..accounts_to_fetch.len() {
                                    if hash_set_known.contains(&accounts_to_fetch[i]) {
                                        if res[i].is_none() {
                                            println!("unable to fetch known account {}", accounts_to_fetch[i]);
                                        }
                                    } else if res[i].is_some() {
                                        println!("fetched unknown account should not be possible");
                                    }
                                }
                            } else {
                                println!("fetched accounts results mismatched");
                            }
                        }
                        Err(e) => {
                            println!("accounts fetching failed because {}", e);
                        }
                    }
                }
            });
            tasks.push(task);
        }

        futures::future::join_all(tasks).await;
        println!(
            "Accounts fetching did {} requests of {} accounts in {} tasks",
            total_fetches.load(std::sync::atomic::Ordering::Relaxed),
            number_of_fetched_accounts,
            NB_ACCOUNT_FETCHING_TASKS
        );

        Ok(())
    }

    fn get_name(&self) -> &'static str {
        "Accounts Fetching"
    }
}
