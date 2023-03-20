use clap::Parser;
use ethers::prelude::*;

const RPC_URL: &str = "https://eth.llamarpc.com";

#[derive(Parser)]
struct Args {
    // Address whose history you want to see.
    address: Address,
    // Block number from which you want to see the history.
    starting_block: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let current_block = provider.get_block_number().await?;

    for block_number in args.starting_block..=current_block.as_u64() {
        let block = provider
            .get_block_with_txs(block_number)
            .await?
            .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?;
        for transaction in block
            .transactions
            .into_iter()
            .filter(|transaction| transaction.from == args.address)
        {
            println!("{:#?}", transaction.hash);
        }
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
