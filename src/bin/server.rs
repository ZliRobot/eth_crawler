use chrono::DateTime;
use clap::Parser;
use ethers::prelude::*;
use std::{error::Error, fmt};

use eth_crawler::modules::*;

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> String {
    let block_number = get_block_number().await;

    match block_number {
        Ok(current_block) => format!("Last block: {}", current_block),
        Err(e) => format!("{}", e),
    }
}

#[get("/balance/<account>/<time>")]
async fn balance(account: String, time: String) -> String {
    let balance = get_balance(account, &time).await;

    match balance {
        Ok(balance) => format!("Balance at {}: {}ETH", time, balance),
        Err(e) => format!("{}", e), //"Error getting balance".into()
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let args = ServerArgs::parse();
    let port = if let Some(port_no) = args.port {
        port_no
    } else {
        8080
    };

    let figment = rocket::Config::figment().merge(("port", port));

    let _rocket = rocket::custom(figment)
        .mount("/", routes![index, balance])
        .launch()
        .await?;

    Ok(())
}

async fn get_block_number() -> Result<U64, ServerError> {
    let provider =
        Provider::<Http>::try_from(RPC_URL).map_err(|_| ServerError::EthClientNotFound)?;
    provider
        .get_block_number()
        .await
        .map_err(|_| ServerError::BlockNotFound)
}

async fn get_balance(address: String, time: &str) -> Result<Balance, ServerError> {
    let time = time.replace("%20", " ");
    println!("{}", time);

    let timestamp = DateTime::parse_from_str(time.trim(), "%Y-%m-%d %H:%M:%S %z")
        .map_err(|_| ServerError::InvalidTimestamp)?
        .timestamp();

    let address = parse_address(&address)?;

    let provider =
        Provider::<Http>::try_from(RPC_URL).map_err(|_| ServerError::EthClientNotFound)?;
    provider
        .balance_at_timestamp(address, timestamp)
        .await
        .map_err(|_| ServerError::BalanceNotAvailable)
}

fn parse_address(address: &str) -> Result<Address, ServerError> {
    let address_digits = address
        .chars()
        .skip(2) //Skip 0x
        .map(|c| {
            c.to_digit(16)
                .map(|d| d as u8)
                .ok_or(ServerError::InvalidAddress)
        })
        .collect::<Result<Vec<_>, ServerError>>()?;

    // Address::try_from(buf) panics if length of buf is not 20!
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

#[derive(Debug)]
enum ServerError {
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
fn test_timestamp() {
    let time = "2022-10-01 08:00:00 +0000";
    DateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S %z")
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
