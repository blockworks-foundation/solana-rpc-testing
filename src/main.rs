pub mod bencher;
mod cli;
mod config;
mod metrics;
mod solana_runtime;
mod test_registry;

use anyhow::{bail, Context};
use clap::Parser;
use cli::Args;
use config::Config;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    assert_ne!(args.threads, 0, "Threads can't be 0");
    assert_ne!(args.duration_in_seconds, 0, "Duration can't be 0");

    let contents = std::fs::read_to_string(&args.config_file)
        .context("Should have been able to read the file")?;
    let config_json: Config = serde_json::from_str(&contents).context("Config file not valid")?;

    if config_json.payers.is_empty() {
        log::error!("config file is missing payers");
        bail!("No payers");
    }

    args.generate_test_registry()
        .start_testing(args, config_json)
        .await;

    Ok(())
}
