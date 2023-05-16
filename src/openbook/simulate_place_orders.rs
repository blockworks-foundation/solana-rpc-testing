use crate::bencher::{Benchmark, Run, Bencher};
use crate::config::{Market, User};
use crate::test_registry::TestingTask;
use crate::utils::noop;
use async_trait::async_trait;
use rand::seq::SliceRandom;
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::compute_budget;
use solana_sdk::hash::Hash;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};
use std::mem::size_of;
use std::{str::FromStr, sync::Arc, time::Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct SimulateOpenbookV2PlaceOrder {
    pub block_hash: Arc<RwLock<Hash>>,
}

#[derive(Serialize, Deserialize)]
#[repr(packed)]
struct PlaceOrderArgs {
    side: u8,
    price_lots: i64,
    max_base_lots: i64,
    max_quote_lots_including_fees: i64,
    client_order_id: u64,
    order_type: u8,
    reduce_only: bool,
    expiry_timestamp: u64,
    limit: u8,
}

trait ToPubkey {
    fn to_pubkey(&self) -> Pubkey;
}

impl ToPubkey for String {
    fn to_pubkey(&self) -> Pubkey {
        Pubkey::from_str(self.as_str()).unwrap()
    }
}

#[async_trait]
impl TestingTask for SimulateOpenbookV2PlaceOrder {
    async fn test(
        &self,
        args: crate::cli::Args,
        config: crate::config::Config,
    ) -> anyhow::Result<()> {
        let openbook_data = config
            .programs
            .iter()
            .find(|x| x.name == "openbook_v2")
            .unwrap()
            .clone();
        let openbook_pid: Pubkey = Pubkey::from_str(openbook_data.program_id.as_str()).unwrap();
        let place_order_cmd = openbook_data
            .commands
            .iter()
            .find(|x| x.name == "placeOrder")
            .unwrap()
            .clone();

        assert!(place_order_cmd.instruction.len() == 8 + size_of::<PlaceOrderArgs>());

        let instant = SimulateOpenbookV2PlaceOrderBench {
            block_hash: self.block_hash.clone(),
            markets: config.markets.clone(),
            users: config.users.clone(),
            place_order_cmd: place_order_cmd.instruction,
            openbook_pid,
        };
        let metric = Bencher::bench::<SimulateOpenbookV2PlaceOrderBench>( instant, args).await?;
        log::info!("{} {}", self.get_name(), serde_json::to_string(&metric)?);
        Ok(())
    }

    fn get_name(&self) -> String {
        format!("Simulating openbook place orders")
    }
}

#[derive(Clone)]
pub struct SimulateOpenbookV2PlaceOrderBench {
    pub block_hash: Arc<RwLock<Hash>>,
    pub markets: Vec<Market>,
    pub users: Vec<User>,
    pub place_order_cmd: Vec<u8>,
    pub openbook_pid: Pubkey,
}


#[async_trait::async_trait]
impl Benchmark for SimulateOpenbookV2PlaceOrderBench {

    async fn run(self, rpc_client: Arc<RpcClient>, duration: std::time::Duration, random_number: u64) -> anyhow::Result<crate::bencher::Run> {
        let mut result = Run::default();

        let mut rng = StdRng::seed_from_u64(random_number);
        let start = Instant::now();
        while start.elapsed() < duration {
            let mut place_order_ix = self.place_order_cmd.clone();
            let market = self.markets.choose(&mut rng).cloned().unwrap();
            let user = self.users.choose(&mut rng).cloned().unwrap();

            let open_orders = user.open_orders[market.market_index]
                    .open_orders
                    .to_pubkey();
            let base_token_account = user.token_data[market.market_index + 1]
                .token_account
                .to_pubkey();
            let quote_token_account = user.token_data[0].token_account.to_pubkey();

            let side = rng.gen_bool(0.5);
            let order_type: u8 = 0;
            let price_diff = 3;
            let price_lots: i64 = if side {
                1000 - price_diff
            } else {
                1000 + price_diff
            };
            let max_base_lots = 5;
            let place_order_params = PlaceOrderArgs {
                client_order_id: 100,
                side: side as u8,
                price_lots,
                max_base_lots,
                max_quote_lots_including_fees: i64::MAX,
                order_type,
                reduce_only: false,
                expiry_timestamp: u64::MAX,
                limit: 10,
            };
            let bytes: Vec<u8> = bincode::serialize(&place_order_params).unwrap();
            assert!(bytes.len() + 8 == place_order_ix.len());

            // copy the instruction data
            for i in 0..bytes.len() {
                place_order_ix[8 + i] = bytes[i];
            }

            let token_account = if side {
                base_token_account
            } else {
                quote_token_account
            };
            let accounts = vec![
                AccountMeta::new(open_orders, false),
                AccountMeta::new(user.pubkey(), false),
                AccountMeta::new(market.market_pk.to_pubkey(), false),
                AccountMeta::new(market.bids.to_pubkey(), false),
                AccountMeta::new(market.asks.to_pubkey(), false),
                AccountMeta::new(token_account, false),
                AccountMeta::new(market.base_vault.to_pubkey(), false),
                AccountMeta::new(market.quote_vault.to_pubkey(), false),
                AccountMeta::new(market.event_queue.to_pubkey(), false),
                AccountMeta::new_readonly(market.oracle.to_pubkey(), false),
                AccountMeta::new_readonly(spl_token::ID, false),
                AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            ];

            let ix = Instruction::new_with_bytes(
                self.openbook_pid,
                place_order_ix.as_slice(),
                accounts,
            );

            let recent_blockhash = *self.block_hash.read().await;

            // to generate new signature each time
            let noop_ix = noop::timestamp();
            // to have higher compute budget limit
            let cu_limits_ix =
                compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(
                    1000000,
                );

            let transaction = Transaction::new(
                &[&user.get_keypair()],
                Message::new(&[noop_ix, cu_limits_ix, ix], Some(&user.pubkey())),
                recent_blockhash,
            );

            match rpc_client.simulate_transaction(&transaction).await {
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
