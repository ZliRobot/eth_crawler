use async_trait::async_trait;
use clap::Parser;
use ethers::prelude::*;

pub const RPC_URL: &str = "https://eth.llamarpc.com";

#[derive(Parser)]
pub struct Args {
    // Address whose history you want to see.
    pub address: Address,
    // Block number from which you want to see the history.
    pub starting_block: u64,
}

pub fn display_header() {
    println!(
        "\n| {:15} | {:10} | {:15} | {:20} |\n{:-<5$}",
        "Tx hash", "Block", "To", "Value", "", 73
    )
}
pub fn print_formated(transaction: Transaction) {
    println!(
        "| {:<15} | {:<10} | {:<15} | {:<20} |",
        transaction.hash.to_string(),
        transaction
            .block_number
            .map(|block_number| block_number.to_string())
            .unwrap_or_else(|| "Pending".into()),
        transaction
            .to
            .map(|to| to.to_string())
            .unwrap_or_else(|| "Creating SC".into()),
        transaction.value
    );
}

#[async_trait]
pub trait EthCrawler {
    async fn transations_from_since_upto(
        self,
        from: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient> EthCrawler for Provider<P> {
    async fn transations_from_since_upto(
        self,
        from: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError> {
        let mut res = vec![];
        for block_number in since_block_number..=upto_block_number {
            let block = self
                .get_block_with_txs(block_number)
                .await?
                .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?;

            for transaction in block
                .transactions
                .into_iter()
                .filter(|transaction| transaction.from == from)
            {
                res.push(transaction);
            }
        }
        Ok(res)
    }
}
