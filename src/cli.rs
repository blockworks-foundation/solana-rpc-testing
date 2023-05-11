use std::{sync::Arc, time::Duration};

use clap::{command, Parser};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

use crate::{
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

    #[arg(short = 'a', long)]
    pub test_all: bool,

    #[arg(short = 'r', long, default_value_t = String::from("http://127.0.0.1:8899"))]
    pub rpc_addr: String,

    #[arg(short = 'd', long, default_value_t = 60)]
    pub duration_in_seconds: u64,

    #[arg(short = 't', long, default_value_t = 4)]
    pub threads: u64,
}

impl Args {
    pub fn generate_test_registry(&self) -> TestRegistry {
        let mut test_registry = TestRegistry::default();

        if self.accounts_fetching || self.test_all {
            test_registry.register(Box::new(AccountsFetchingTests));
        }

        if self.send_and_confirm_transaction || self.test_all {
            test_registry.register(Box::new(SendAndConfrimTesting));
        }

        if self.get_slot || self.test_all {
            test_registry.register(Box::new(GetSlotTest));
        }

        if self.get_block || self.test_all {
            test_registry.register(Box::new(GetBlockTest));
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
