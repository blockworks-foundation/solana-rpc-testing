use std::collections::HashMap;

use crate::{bencher::Stats, cli::Args, config::Config};
use async_trait::async_trait;

#[async_trait]
pub trait TestingTask: Send + Sync {
    async fn test(&self, args: &Args, config: &Config) -> anyhow::Result<Stats>;
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

    pub async fn run_tests(&self, args: &Args, config: &Config) -> HashMap<String, Stats> {
        let mut results = HashMap::new();

        for test in &self.tests {
            let name = test.get_name();

            log::info!("Test: {name}");

            match test.test(args, config).await {
                Ok(stat) => {
                    log::info!("Test {name} passed");
                    log::info!("Metric:\n{stat:#?}");

                    results.insert(name, stat);
                }
                Err(e) => log::error!("test {name} failed with error {e}"),
            }

        }

        results
    }
}
