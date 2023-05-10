use async_trait::async_trait;

use crate::{cli::Args, config::Config, metrics::Metrics};

#[async_trait]
pub trait TestingTask: Send + Sync {
    async fn test(&self, args: Args, config: Config) -> anyhow::Result<Metrics>;
    fn get_name(&self) -> &'static str;
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
        let tasks = self.tests.into_iter().map(|test| {
            let args = args.clone();
            let config = config.clone();

            tokio::spawn(async move {
                log::info!("test {}", test.get_name());

                match test.test(args, config).await {
                    Ok(_) => {
                        println!("test {} passed", test.get_name());
                    }
                    Err(e) => {
                        println!("test {} failed with error {}", test.get_name(), e);
                    }
                }
            })
        });

        futures::future::join_all(tasks).await;
    }
}
