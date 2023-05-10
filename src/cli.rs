use std::sync::Arc;
use std::time::Duration;

use clap::{command, Parser};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

use crate::solana_runtime::accounts_fetching::AccountsFetchingTests;
use crate::solana_runtime::get_block::GetBlockTest;
use crate::solana_runtime::get_slot::GetSlotTest;
use crate::solana_runtime::send_and_get_status_memo::SendAndConfrimTesting;
use crate::test_registry::TestRegistry;

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
    pub duration_in_seconds: usize,
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
        Duration::from_secs(self.duration_in_seconds as u64)
    }

    #[inline]
    pub fn get_rpc_client(&self) -> Arc<RpcClient> {
        Arc::new(RpcClient::new(self.rpc_addr.clone()))
    }
}
