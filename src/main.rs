mod cli;
mod config;
mod solana_runtime;
mod test_registry;

use cli::Args;
use config::Config;

use clap::Parser;
use test_registry::TestRegistry;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config_file = args.config_file.clone();

    let contents =
        std::fs::read_to_string(config_file).expect("Should have been able to read the file");

    let config_json: Config =
        serde_json::from_str(contents.as_str()).expect("Config file not valid");

    if config_json.payers.is_empty() {
        log::error!("config file is missing payers");
        return Err(anyhow::Error::msg("No payers"));
    }

    let mut registry = TestRegistry::new();
    registry.register_all();
    registry.start_testing(args, config_json).await;
    Ok(())
}
