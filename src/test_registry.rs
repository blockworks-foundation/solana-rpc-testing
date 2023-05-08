use async_trait::async_trait;

use crate::{
    cli::Args,
    config::Config,
    solana_runtime::{
        accounts_fetching::AccountsFetchingTests, send_and_get_status_memo::SendAndConfrimTesting,
    },
};
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
        self.register(Arc::new(AccountsFetchingTests {}));
        self.register(Arc::new(SendAndConfrimTesting {}));
    }

    pub async fn start_testing(&self, args: Args, config: Config) {
        let mut tasks = vec![];
        for test in &self.tests {
            let test = test.clone();
            let args = args.clone();
            let config = config.clone();
            let task = tokio::spawn(async move {
                match test.test(args, config).await {
                    Ok(_) => {
                        println!("test {} passed", test.get_name());
                    }
                    Err(e) => {
                        println!("test {} failed with error {}", test.get_name(), e);
                    }
                }
            });
            tasks.push(task);
        }
        futures::future::join_all(tasks).await;
    }
}
