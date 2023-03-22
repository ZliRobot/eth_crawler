use chrono::DateTime;
use clap::Parser;
use ethers::prelude::*;
use std::sync::Arc;

mod lib;
use lib::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let provider = Arc::new(Provider::<Http>::try_from(RPC_URL)?);
    println!("Connected to: {}", RPC_URL);
    let current_block = provider.get_block_number().await?;
    println!("Current block number: {}", current_block);

    if let Some(starting_block) = args.starting_block {
        display_header();
        for transaction in provider
            .clone()
            .transations_of_since_upto(args.address, starting_block, current_block.as_u64())
            .await?
            .into_iter()
        {
            print_formated(transaction, args.address);
        }
    }

    if let Some(time) = args.time {
        let timestamp = DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S %z")?.timestamp();
        println!(
            "Balance at {}: {}ETH",
            time,
            provider
                .balance_at_timestamp(args.address, timestamp)
                .await?
        );
    }

    Ok(())
}
