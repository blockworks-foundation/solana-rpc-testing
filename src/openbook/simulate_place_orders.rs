use crate::test_registry::TestingTask;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use solana_sdk::hash::Hash;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};
use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};
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
        let mut tasks = vec![];
        let openbook_data = config
            .programs
            .iter()
            .find(|x| x.name == "openbook_v2")
            .unwrap()
            .clone();
        let openbook_pk = Pubkey::from_str(openbook_data.program_id.as_str()).unwrap();
        let place_order_cmd = openbook_data
            .commands
            .iter()
            .find(|x| x.name == "placeOrder")
            .unwrap()
            .clone();

        assert!(place_order_cmd.instruction.len() == 8 + size_of::<PlaceOrderArgs>());
        let successful_orders_count = Arc::new(AtomicU64::new(0));

        for user in &config.users {
            for market in &config.markets {
                let market = market.clone();
                let user = user.clone();
                let args = args.clone();
                let place_order_cmd = place_order_cmd.clone();
                let openbook_pk = openbook_pk.clone();
                let block_hash = self.block_hash.clone();
                let open_orders = user.open_orders[market.market_index]
                    .open_orders
                    .to_pubkey();
                let base_token_account = user.token_data[market.market_index + 1]
                    .token_account
                    .to_pubkey();
                let quote_token_account = user.token_data[0].token_account.to_pubkey();
                let successful_orders_count = successful_orders_count.clone();

                let task = tokio::spawn(async move {
                    let start = Instant::now();

                    let mut place_order_ix = place_order_cmd.instruction;
                    let rpc_client = args.get_rpc_client();
                    let mut side = false;
                    let mut price_diff: i64 = 1;
                    let mut max_base_lots = 1;
                    while start.elapsed().as_secs() < args.duration_in_seconds {
                        let order_type: u8 = 0;

                        let price_lots: i64 = if side {
                            1000 - price_diff
                        } else {
                            1000 + price_diff
                        };
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
                            openbook_pk,
                            place_order_ix.as_slice(),
                            accounts,
                        );

                        let recent_blockhash = *block_hash.read().await;
                        let transaction = Transaction::new(
                            &[&user.get_keypair()],
                            Message::new(&[ix], Some(&user.pubkey())),
                            recent_blockhash,
                        );
                        let signature = transaction.signatures[0];
                        if let Err(e) = rpc_client.simulate_transaction(&transaction).await {
                            log::error!(
                                "error while simulating openbook place order {} sig {}",
                                e,
                                signature
                            );
                        } else {
                            successful_orders_count.fetch_add(1, Ordering::Relaxed);
                        }

                        // update side and price diff
                        side = !side;
                        price_diff = price_diff % 6 + 1;
                        max_base_lots = max_base_lots % 10 + 1;
                    }
                });
                tasks.push(task);
            }
        }

        futures::future::join_all(tasks).await;

        log::info!(
            "Simulated {} transactions with {} users and {} markets",
            successful_orders_count.load(Ordering::Relaxed),
            config.users.len(),
            config.markets.len()
        );
        Ok(())
    }

    fn get_name(&self) -> String {
        format!("Simulating openbook place orders")
    }
}
