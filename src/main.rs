use clap::Parser;
use ethers::prelude::*;

mod lib;
use crate::lib::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let current_block = provider.get_block_number().await?;
    display_header();

    for transaction in provider
        .transations_from_since_upto(args.address, args.starting_block, current_block.as_u64())
        .await?
        .into_iter()
    {
        print_formated(transaction);
    }

    Ok(())
}

#[cfg(test)]
#[tokio::test]
async fn print_current_block_number() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let current_block = provider.get_block_number().await?;
    println!("Current block number is: {}", current_block);
    Ok(())
}
