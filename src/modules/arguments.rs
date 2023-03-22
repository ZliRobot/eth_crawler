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

#[derive(Parser)]
pub struct ServerArgs {
    /// Port number to listen on. (Default: 8080)
    #[arg(short, long)]
    pub port: Option<u64>,
}
