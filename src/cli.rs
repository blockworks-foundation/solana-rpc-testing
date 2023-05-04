use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'c', long, default_value_t = String::from("config.json"))]
    pub config_file: String,

    #[arg(long)]
    pub accounts_fetching: bool,

    #[arg(long)]
    pub send_and_confirm_transaction : bool,

    #[arg(short = 'a', long)]
    pub test_all: bool,
}

impl Args {
    pub fn test_accounts_fetching(&self) -> bool {
        self.accounts_fetching || self.test_all
    }

    pub fn test_send_and_confirm_transactions(&self) -> bool {
        self.send_and_confirm_transaction || self.test_all
    }
}