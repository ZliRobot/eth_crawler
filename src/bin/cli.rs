use chrono::DateTime;
use clap::Parser;
use ethers::prelude::*;
use std::sync::Arc;

use eth_crawler::{cli::*, provider::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let provider = Arc::new(Provider::<Http>::try_from(RPC_URL)?);
    println!("Connected to: {}", RPC_URL);
    let current_block = provider.get_block_number().await?;
    println!("Current block number: {}", current_block);

    match args {
        CliArgs {
            address,
            starting_block: Some(starting_block),
            time: _,
        } => {
            display_header();
            for transaction in provider
                .transations_of_since_upto(address, starting_block, current_block.as_u64())
                .await?
                .into_iter()
            {
                print_formated(transaction, address);
            }
        }

        CliArgs {
            address,
            starting_block: _,
            time: Some(time),
        } => {
            let timestamp = DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S %z")?.timestamp();
            if provider.get_block_timestamp(current_block).await? < timestamp as u64 {
                println!("Unfortunatly, this app can't predict the future");
            } else {
                println!(
                    "Balance at {}: {}ETH",
                    time,
                    provider.balance_at_timestamp(address, timestamp).await?
                );
            }
        }
        _ => {}
    }

    Ok(())
}
