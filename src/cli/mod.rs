pub mod display;
pub use display::{display_header, print_formated};

use clap::Parser;
use ethers::prelude::*;

#[derive(Parser)]
pub struct CliArgs {
    /// Address whose history you want to see.
    pub address: Address,

    /// Block number starting from which you want to see the history.
    #[arg(short, long)]
    pub starting_block: Option<u64>,

    /// Time at which you want to see the balance.
    #[arg(short, long)]
    pub time: Option<String>,
}
