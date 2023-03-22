use async_trait::async_trait;
use ethers::prelude::*;
use std::sync::Arc;
use tokio;

#[async_trait]
pub trait EthCrawlerTransactions {
    async fn transations_of_since_upto(
        self,
        from: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient + 'static> EthCrawlerTransactions for Arc<Provider<P>> {
    async fn transations_of_since_upto(
        self,
        target_addr: H160,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError> {
        let target_addr = Arc::new(target_addr);

        let tasks = (since_block_number..=upto_block_number)
            .map(|block_number| {
                let provider = self.clone();
                let target_addr = target_addr.clone();

                tokio::spawn(async move {
                    Ok(provider
                        .get_block_with_txs(block_number)
                        .await?
                        .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?
                        .transactions
                        .into_iter()
                        .filter(move |transaction| {
                            transaction.from == *target_addr || transaction.to == Some(*target_addr)
                        }))
                })
            })
            .collect::<Vec<_>>();

        let transactions = futures::future::try_join_all(tasks)
            .await
            .map_err(|e| ProviderError::CustomError(format!("{:?}", e)))?
            .into_iter()
            .collect::<Result<Vec<_>, ProviderError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Ok(transactions)
    }
}
