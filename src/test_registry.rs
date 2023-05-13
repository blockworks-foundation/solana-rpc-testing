use async_trait::async_trait;
use crate::{cli::Args, config::Config};

#[async_trait]
pub trait TestingTask: Send + Sync {
    async fn test(&self, args: Args, config: Config) -> anyhow::Result<()>;
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
        
        let tasks = self.tests.into_iter().map(|test| {
            let args = args.clone();
            let config = config.clone();
            let name = test.get_name();

            tokio::spawn(async move {
                log::info!("test {name}");

                match test.test(args, config).await {
                    Ok(_) => log::info!("test {name} passed"),
                    Err(e) => log::info!("test {name} failed with error {e}"),
                }
            })
        });

        futures::future::join_all(tasks).await;
    }
}
