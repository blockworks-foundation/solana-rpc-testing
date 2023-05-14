use std::{sync::Arc, time::Duration};

use clap::{command, Parser};
use futures::StreamExt;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient, rpc_config::RpcTransactionLogsConfig,
};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use tokio::sync::RwLock;

use crate::{
    openbook::simulate_place_orders::SimulateOpenbookV2PlaceOrder,
    solana_runtime::{
        accounts_fetching::AccountsFetchingTests, get_block::GetBlockTest, get_slot::GetSlotTest,
        send_and_get_status_memo::SendAndConfrimTesting,
    },
    test_registry::TestRegistry,
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'c', long, default_value_t = String::from("configure/config.json"))]
    pub config_file: String,

    #[arg(long)]
    pub accounts_fetching: bool,

    #[arg(long)]
    pub send_and_confirm_transaction: bool,

    #[arg(long)]
    pub get_slot: bool,

    #[arg(long)]
    pub get_block: bool,

    #[arg(long)]
    pub simulate_openbook_v2: bool,

    #[arg(short = 'a', long)]
    pub test_all: bool,

    #[arg(short = 'r', long, default_value_t = String::from("http://127.0.0.1:8899"))]
    pub rpc_addr: String,

    #[arg(short = 'w', long, default_value_t = String::from("ws://127.0.0.1:8900"))]
    pub rpc_ws_addr: String,

    #[arg(short = 'd', long, default_value_t = 60)]
    pub duration_in_seconds: u64,

    #[arg(short = 't', long, default_value_t = 32)]
    pub threads: u64,

    #[arg(short = 'p', long)]
    pub print_logs: bool,
}

impl Args {
    pub fn generate_test_registry(&self, block_hash: Arc<RwLock<Hash>>) -> TestRegistry {
        if self.print_logs {
            let rpc_ws_addr = self.rpc_ws_addr.clone();
            tokio::spawn(async move {
                let pubsub_client = PubsubClient::new(&rpc_ws_addr).await.unwrap();
                let res = pubsub_client
                    .logs_subscribe(
                        solana_client::rpc_config::RpcTransactionLogsFilter::All,
                        RpcTransactionLogsConfig { commitment: None },
                    )
                    .await;
                match res {
                    Ok((mut stream, _)) => loop {
                        let log = stream.next().await;
                        match log {
                            Some(log) => {
                                for log_s in log.value.logs {
                                    println!("{}", log_s);
                                }
                            }
                            None => {}
                        }
                    },
                    Err(e) => {
                        println!("error subscribing to the logs {}", e);
                    }
                }
            });
        }

        let mut test_registry = TestRegistry::default();

        if self.accounts_fetching || self.test_all {
            test_registry.register(Box::new(AccountsFetchingTests));
        }

        if self.send_and_confirm_transaction || self.test_all {
            test_registry.register(Box::new(SendAndConfrimTesting {
                block_hash: block_hash.clone(),
            }));
        }

        if self.get_slot || self.test_all {
            test_registry.register(Box::new(GetSlotTest));
        }

        if self.get_block || self.test_all {
            test_registry.register(Box::new(GetBlockTest));
        }

        if self.simulate_openbook_v2 || self.test_all {
            test_registry.register(Box::new(SimulateOpenbookV2PlaceOrder { block_hash }));
        }

        test_registry
    }

    #[inline]
    pub fn get_duration_to_run_test(&self) -> Duration {
        Duration::from_secs(self.duration_in_seconds)
    }

    #[inline]
    pub fn get_rpc_client(&self) -> Arc<RpcClient> {
        Arc::new(RpcClient::new(self.rpc_addr.clone()))
    }
}
