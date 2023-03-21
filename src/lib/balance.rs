use async_trait::async_trait;
use std::convert::From;
use std::fmt;
use ethers::prelude::*;

pub struct Balance{
    wei: U256
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:0>19}", self.wei).chars().rev().collect::<Vec<_>>();
        s.insert(18, '.');
        let s = s.into_iter().rev().collect::<String>();
        std::write!(f, "{}", s)
    }
}

impl From<U256> for Balance {
    fn from(wei: U256) -> Self {
        Self{wei}
    }
}

#[async_trait]
pub trait EthCrawlerBalance {
    async fn balance_at_timestamp(
        &self,
        address: H160,
        time: i64,
    ) -> Result<Balance, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient> EthCrawlerBalance for Provider<P> {
    async fn balance_at_timestamp(
        &self,
        address: H160,
        time: i64,
    ) -> Result<Balance, ProviderError> {
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

        Ok(Balance{ wei: self.get_balance(address, Some(lower_block.into())).await?} )
    }
}