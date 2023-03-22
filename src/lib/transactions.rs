use async_trait::async_trait;
use ethers::prelude::*;

#[async_trait]
pub trait EthCrawlerTransactions {
    async fn transations_of_since_upto(
        &self,
        from: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient> EthCrawlerTransactions for Provider<P> {
    async fn transations_of_since_upto(
        &self,
        target_addr: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError> {
        let mut res = vec![];
        for block_number in since_block_number..=upto_block_number {
            res.push(self
                .get_block_with_txs(block_number)
                .await?
                .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?
                .transactions
                .into_iter()
                .filter(|transaction| {
                    transaction.from == target_addr || transaction.to == Some(target_addr)
                }));
        }
        
        Ok(res.into_iter().flatten().collect::<Vec<_>>())
    }
}
