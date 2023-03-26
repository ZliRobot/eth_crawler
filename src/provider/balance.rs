use super::repeat_if_network_error;
use async_trait::async_trait;
use ethers::prelude::*;
use std::convert::From;
use std::fmt;

pub struct Balance {
    wei: U256,
}

impl fmt::Display for Balance {
    /// Returns a string representation of the balance in ETH
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:0>19}", self.wei.to_string())
            .chars()
            .rev()
            .collect::<Vec<_>>();
        s.insert(18, '.');
        let s = s.into_iter().rev().collect::<String>();
        std::write!(f, "{}", s)
    }
}

impl From<U256> for Balance {
    fn from(wei: U256) -> Self {
        Self { wei }
    }
}

#[async_trait]
pub trait EthCrawlerBalance {
    async fn balance_at_timestamp(
        &self,
        address: Address,
        timestamp: i64,
    ) -> Result<Balance, ProviderError>;

    async fn get_block_timestamp(&self, block_id: U64) -> Result<u64, ProviderError>;
}

#[async_trait]
impl<P: JsonRpcClient + 'static> EthCrawlerBalance for Provider<P> {
    async fn balance_at_timestamp(
        &self,
        address: Address,
        timestamp: i64,
    ) -> Result<Balance, ProviderError> {
        let target_time = timestamp as u64;
        let mut upper_block = self.get_block_number().await?;
        let mut lower_block: U64 = 0.into();
        let mut current_block = (upper_block + lower_block) / 2;
        let mut current_block_time = self.get_block_timestamp(current_block).await?;

        // There were no balances before block 0
        if timestamp < self.get_block_timestamp(0.into()).await? as i64 {
            return Ok(Balance::from(U256::zero()));
        }

        // Find the required block using binary search
        while upper_block > lower_block + U64::from(1) {
            if current_block_time > target_time {
                upper_block = current_block;
            } else {
                lower_block = current_block;
            }
            current_block = (lower_block + upper_block) / 2;
            current_block_time = self.get_block_timestamp(current_block).await?;
        }

        Ok(Balance {
            wei: self.get_balance(address, Some(lower_block.into())).await?,
        })
    }

    async fn get_block_timestamp(&self, block_id: U64) -> Result<u64, ProviderError> {
        let block = repeat_if_network_error(&Provider::get_block, self, block_id).await;

        Ok(block?
            .ok_or_else(|| ProviderError::CustomError("Block unavailable".into()))?
            .timestamp
            .as_u64())
    }
}

#[cfg(test)]
use std::io::Write;
#[test]
fn test_print_balance() {
    let wei: U256 = 0.into();

    let mut w = Vec::new();
    write!(w, "{}", Balance { wei }).unwrap();
    assert_eq!(w, "0.000000000000000000".as_bytes().to_vec());
}
