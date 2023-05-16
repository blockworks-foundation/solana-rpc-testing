use std::{collections::HashMap, sync::Arc};

use crate::{bencher::Stats, cli::Args, config::Config};
use async_trait::async_trait;
use tokio::sync::RwLock;

#[async_trait]
pub trait TestingTask: Send + Sync {
    async fn test(&self, args: Args, config: Config) -> anyhow::Result<Stats>;
    fn get_name(&self) -> String;
}

#[derive(Default)]
pub struct TestRegistry {
    tests: Vec<Box<dyn TestingTask>>,
}

impl TestRegistry {
    pub fn register(&mut self, test: Box<dyn TestingTask>) {
        self.tests.push(test);
    }

    pub async fn start_testing(self, args: Args, config: Config) {
        let results = Arc::new(RwLock::new(HashMap::new()));
        let tasks = self.tests.into_iter().map(|test| {
            let args = args.clone();
            let config = config.clone();
            let name = test.get_name();
            let results = results.clone();

            tokio::spawn(async move {
                log::info!("test {name}");

                match test.test(args, config).await {
                    Ok(metric) => {
                        log::info!("test {name} passed");
                        let mut lock = results.write().await;
                        lock.insert(test.get_name(), metric);
                    }
                    Err(e) => log::info!("test {name} failed with error {e}"),
                }
            })
        });

        futures::future::join_all(tasks).await;
        let res = results.read().await.clone();
        if !args.output_file.is_empty() {
            let result_string = serde_json::to_string(&res);
            if let Ok(result) = result_string {
                std::fs::write(args.output_file, result).expect("Could not write output file");
            }
        }
    }
}
