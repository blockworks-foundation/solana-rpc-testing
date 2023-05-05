use async_trait::async_trait;

use crate::{cli::Args, config::Config, solana_runtime::accounts_fetching::AccountsFetchingTests};
use std::sync::Arc;

#[async_trait]
pub trait TestingTask: Send + Sync {
    async fn test(&self, args: Args, config: Config) -> anyhow::Result<()>;
    fn get_name(&self) -> String;
}

pub struct TestRegistry {
    tests: Vec<Arc<dyn TestingTask>>,
}

impl TestRegistry {
    pub fn new() -> Self {
        Self { tests: vec![] }
    }

    fn register(&mut self, test: Arc<dyn TestingTask>) {
        self.tests.push(test);
    }

    pub fn register_all(&mut self) {
        self.register(Arc::new(AccountsFetchingTests {}))
    }

    pub async fn start_testing(&self, args: Args, config: Config) {
        for test in &self.tests {
            match test.test(args.clone(), config.clone()).await {
                Ok(_) => {
                    log::info!("test {} passed", test.get_name());
                }
                Err(e) => {
                    log::error!("test {} failed with error {}", test.get_name(), e);
                }
            }
        }
    }
}
