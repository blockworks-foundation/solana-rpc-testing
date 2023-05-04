mod cli;
mod solana_runtime;
mod config;

use cli::{Args};
use config::Config;

use clap::Parser;

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config_file = args.config_file;

    let contents = std::fs::read_to_string(config_file)
        .expect("Should have been able to read the file");
    
    let config_json: Config = serde_json::from_str(contents.as_str())?;

    Ok(())
}
