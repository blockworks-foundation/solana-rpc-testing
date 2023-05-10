use crate::metrics::Metrics;
use crate::{cli::Args, config::Config, test_registry::TestingTask};

pub struct GetBlockTest;

#[async_trait::async_trait]
impl TestingTask for GetBlockTest {
    async fn test(&self, args: Args, _config: Config) -> anyhow::Result<Metrics> {
        let rpc_client = args.get_rpc_client();

        let futs = (0..12).map(|_| {
            let rpc_client = rpc_client.clone();

            tokio::spawn(async move { rpc_client.get_slot().await })
        });

        let _slots = futures::future::try_join_all(futs).await?;

        Ok(Default::default())
    }

    fn get_name(&self) -> &'static str {
        "GetBlockTest"
    }
}
