use clap::Parser;
use ethers::prelude::*;

pub mod balance;
pub mod display;
pub mod transactions;

pub use balance::EthCrawlerBalance;
pub use display::{display_header, print_formated};
pub use transactions::EthCrawlerTransactions;

pub const RETRY_COUNT: usize = 4;
pub const RPC_URL: &str = "https://eth.llamarpc.com";

#[derive(Parser)]
pub struct Args {
    /// Address whose history you want to see.
    pub address: Address,

    /// Block number starting from which you want to see the history.
    #[arg(short, long)]
    pub starting_block: Option<u64>,

    /// Time at which you want to see the balance.
    #[arg(short, long)]
    pub time: Option<String>,
}
