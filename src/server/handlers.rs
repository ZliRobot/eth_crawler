use chrono::naive::NaiveDateTime;
use ethers::{prelude::*, providers::Http};
use std::sync::Arc;
use std::{error::Error, fmt};

use crate::provider::*;

pub async fn get_last_block_number(provider: &Arc<Provider<Http>>) -> Result<U64, ServerError> {
    provider
        .get_block_number()
        .await
        .map_err(|_| ServerError::BlockNotFound)
}

pub async fn get_balance(
    provider: &Arc<Provider<Http>>,
    address: String,
    time: &str,
) -> Result<Balance, ServerError> {
    let timestamp = NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M"))
        .map_err(|_| ServerError::InvalidTimestamp)?
        .timestamp();

    let address = parse_address(&address)?;

    provider
        .balance_at_timestamp(address, timestamp)
        .await
        .map_err(|_| ServerError::BalanceNotAvailable)
}

pub async fn get_transactions(
    provider: &Arc<Provider<Http>>,
    address: Address,
    starting_block: u64,
) -> Result<Vec<Transaction>, ServerError> {
    let current_block = get_last_block_number(provider).await?.as_u64();

    provider
        .transations_of_since_upto(address, starting_block, current_block)
        .await
        .map_err(|_| ServerError::BlockNotFound)
}

pub fn parse_address(address: &str) -> Result<Address, ServerError> {
    let address_digits = address
        .chars()
        .skip_while(|&c| c != 'x') // skip eventual leading zeros and whitespaces
        .skip(1) // Skip x
        .map(|c| {
            c.to_digit(16)
                .map(|d| d as u8)
                .ok_or(ServerError::InvalidAddress)
        })
        .collect::<Result<Vec<_>, ServerError>>()?;

    // Address::try_from(buf) panics if length of buf is not 20!
    if address_digits.len() != 40 {
        return Err(ServerError::InvalidAddress);
    }
    let mut buf = [0u8; 20];
    for i in 0..40 {
        buf[i / 2] |= if i % 2 == 0 {
            address_digits[i] << 4
        } else {
            address_digits[i]
        };
    }
    Address::try_from(&buf).map_err(|_| ServerError::InvalidAddress)
}

#[derive(Debug, PartialEq)]
pub enum ServerError {
    EthClientNotFound,
    BlockNotFound,
    BalanceNotAvailable,
    InvalidTimestamp,
    InvalidAddress,
}

impl Error for ServerError {}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::EthClientNotFound => write!(f, "Eth Client Not Found"),
            ServerError::BlockNotFound => write!(f, "Block Not Found"),
            ServerError::BalanceNotAvailable => write!(f, "Balance Not Available"),
            ServerError::InvalidTimestamp => write!(f, "Invalid Timestamp"),
            ServerError::InvalidAddress => write!(f, "Invalid Address"),
        }
    }
}

#[cfg(test)]
#[test]
fn test_timestamp_parsing() {
    let time = "2023-03-14T08:55:04";
    NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S")
        .unwrap()
        .timestamp();
}

#[test]
fn test_short_timestamp_parsing() {
    let time = "2023-03-14T08:55";
    NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M")
        .unwrap()
        .timestamp();
}

#[test]
fn test_parse_address() -> Result<(), ServerError> {
    let address = H160::random();
    println!("address: {:?}", address);
    println!("parsed: {:?}", parse_address(&format!("{:?}", address))?);
    assert_eq!(parse_address(&format!("{:?}", address))?, address);
    Ok(())
}

#[test]
fn test_parse_invalid_address() -> Result<(), ServerError> {
    let address = H160::random();
    let invalid_address = &format!("{:?}", address)[0..19];
    assert_eq!(
        parse_address(invalid_address),
        Err(ServerError::InvalidAddress)
    );

    Ok(())
}
