use async_trait::async_trait;
use clap::Parser;
use ethers::prelude::*;

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

pub fn display_header() {
    println!(
        "\n| {:15} | {:10} | {:20} | {:20} |\n{:-<5$}",
        "Tx hash", "Block", "To/from", "Value", "", 78
    )
}
pub fn print_formated(transaction: Transaction, target: Address) {
    let (direction, address) = if transaction.from == target {
        (
            "to".to_string(),
            transaction
                .to
                .map(|to| to.to_string())
                .unwrap_or_else(|| "Creating SC".into()),
        )
    } else {
        ("from".into(), transaction.from.to_string())
    };

    println!(
        "| {:<15} | {:<10} | {:4} {:<15} | {:<20} |",
        transaction.hash.to_string(),
        transaction
            .block_number
            .map(|block_number| block_number.to_string())
            .unwrap_or_else(|| "Pending".into()),
        direction,
        address,
        transaction.value
    );
}

#[async_trait]
pub trait EthCrawler {
    async fn transations_of_since_upto(
        &self,
        from: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError>;

    async fn balance_at_timestamp(
        &self,
        address: H160,
        time: i64,
    ) -> Result<U256, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient> EthCrawler for Provider<P> {
    async fn transations_of_since_upto(
        &self,
        target_addr: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError> {
        let mut res = vec![];
        for block_number in since_block_number..=upto_block_number {
            let block = self
                .get_block_with_txs(block_number)
                .await?
                .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?;

            for transaction in block.transactions.into_iter().filter(|transaction| {
                transaction.from == target_addr || transaction.to == Some(target_addr)
            }) {
                res.push(transaction);
            }
        }
        Ok(res)
    }

    async fn balance_at_timestamp(
        &self,
        address: H160,
        time: i64,
    ) -> Result<U256, ProviderError> {
        let target_time = time as u64;
        let mut upper_block = self.get_block_number().await?;
        let mut lower_block: U64 = 0.into();
        let mut current_block = (upper_block + lower_block) / 2;
        let mut current_block_time = self
            .get_block(current_block)
            .await?
            .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?
            .timestamp
            .as_u64();

        while upper_block > lower_block + U64::from(1) {
            if current_block_time > target_time {
                upper_block = current_block;
            } else {
                lower_block = current_block;
            }
            current_block = (lower_block + upper_block) / 2;
            current_block_time = self
                .get_block(current_block)
                .await?
                .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?
                .timestamp
                .as_u64();
        }

        Ok(self.get_balance(address, Some(lower_block.into())).await?)
    }
}
