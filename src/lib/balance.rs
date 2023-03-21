use async_trait::async_trait;
use ethers::prelude::*;

#[async_trait]
pub trait EthCrawlerBalance {
    async fn balance_at_timestamp(
        &self,
        address: H160,
        time: i64,
    ) -> Result<U256, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient> EthCrawlerBalance for Provider<P> {
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