use async_trait::async_trait;
use ethers::prelude::*;
use std::sync::Arc;
use tokio;

use super::repeat_if_network_error;

#[async_trait]
pub trait EthCrawlerTransactions {
    async fn transations_of_since_upto(
        &self,
        address: Address,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient + 'static> EthCrawlerTransactions for Arc<Provider<P>> {
    async fn transations_of_since_upto(
        &self,
        address: Address,
        since_block_number: u64,
        upto_block_number: u64,
    ) -> Result<Vec<Transaction>, ProviderError> {
        let address = Arc::new(address);

        let tasks = (since_block_number..=upto_block_number)
            .map(|block_number| {
                let provider = self.clone();
                let address = address.clone();

                tokio::spawn(get_block_transactions(provider, address, block_number))
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

async fn get_block_transactions<P: JsonRpcClient>(
    provider: Arc<Provider<P>>,
    address: Arc<Address>,
    block_number: u64,
) -> Result<Vec<Transaction>, ProviderError> {
    let block = repeat_if_network_error(
        &Provider::get_block_with_txs,
        &provider,
        block_number.into(),
    )
    .await;

    Ok(block?
        .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?
        .transactions
        .into_iter()
        .filter(move |transaction| transaction.from == *address || transaction.to == Some(*address))
        .collect::<Vec<_>>())
}

pub fn direction_address(transaction: &Transaction, target: Address) -> (String, String) {
    if transaction.from == target {
        (
            "to".to_string(),
            transaction
                .to
                .map(|to| format!("{:?}", to))
                .unwrap_or_else(|| "Creating SC".into()),
        )
    } else {
        ("from".into(), format!("{:?}", transaction.from))
    }
}
