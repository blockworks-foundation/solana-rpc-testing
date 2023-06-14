use std::sync::atomic::{AtomicU64, Ordering};

use bytes::Bytes;
use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Value};
use solana_client::rpc_request::RpcRequest;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client::rpc_client::SerializableTransaction;

use crate::bencher::Run;

#[derive(derive_more::Deref)]
pub struct CustomRpcClient {
    client: reqwest::Client,
    url: String,
    id: AtomicU64,
    metric: Run,
    #[deref]
    rpc_client: RpcClient,
}

// Don't transfer id and bytes_sent
impl Clone for CustomRpcClient {
    fn clone(&self) -> Self {
        Self::new(self.url.clone())
    }
}

impl From<CustomRpcClient> for Run {
    fn from(val: CustomRpcClient) -> Self {
        val.metric
    }
}

impl CustomRpcClient {
    pub fn new(url: String) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            rpc_client: RpcClient::new(url.clone()),
            url,
            id: 1.into(),
            metric: Run::default(),
        }
    }

    pub async fn raw_get_slot(&mut self) {
        self.send(RpcRequest::GetSlot, Value::Null).await
    }

    pub async fn serialize_tx(tx: impl SerializableTransaction) -> String {
        let tx = bincode::serialize(&tx).unwrap();
        bs58::encode(tx).into_string()
    }

    pub async fn raw_get_block(&mut self, slot: impl Into<u64>) {
        self.send(RpcRequest::GetBlock, json! {[slot.into()]}).await
    }

    pub async fn raw_get_multiple_accounts(&mut self, accounts: Vec<Pubkey>) {
        let accounts: Vec<String> = accounts
            .into_iter()
            .map(|pubkey| pubkey.to_string())
            .collect();

        self.send(RpcRequest::GetMultipleAccounts, json!([accounts]))
            .await
    }

    pub async fn raw_send_transaction(&mut self, tx: impl SerializableTransaction) {
        let tx = Self::serialize_tx(tx).await;

        self.send(RpcRequest::SendTransaction, json! {[tx]}).await
    }

    pub async fn raw_simulate_transaction(&mut self, tx: impl SerializableTransaction) {
        let tx = Self::serialize_tx(tx).await;

        self.send(RpcRequest::SimulateTransaction, json! {[tx]})
            .await
    }

    pub async fn send(&mut self, request: RpcRequest, params: Value) {
        let id = self.id.fetch_add(1, Ordering::Relaxed);

        let req_raw_body = request
            .build_request_json(id, params)
            .to_string()
            .into_bytes();

        let bytes_sent = req_raw_body.len();
        self.metric.bytes_sent += bytes_sent as u64;

        let err = match self.send_raw(req_raw_body).await {
            Ok(res_bytes) => {
                self.metric.bytes_received += res_bytes.len() as u64;

                let res: Value =
                    serde_json::from_slice(&res_bytes).expect("Server invalid response json");

                if res.get("result").is_some() {
                    self.metric.requests_completed += 1;
                    return;
                }

                res["error"].to_string()
            }
            Err(err) => err.to_string(),
        };

        self.metric.errors.push(err);
        self.metric.requests_failed += 1;
    }

    pub async fn send_raw(&self, req_raw_body: Vec<u8>) -> anyhow::Result<Bytes> {
        Ok(self
            .client
            .post(&self.url)
            .header(CONTENT_TYPE, "application/json")
            .body(req_raw_body)
            .send()
            .await?
            .bytes()
            .await?)
    }
}
