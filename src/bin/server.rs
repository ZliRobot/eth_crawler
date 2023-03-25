use chrono::naive::NaiveDateTime;
use clap::Parser;
use ethers::{prelude::*, providers::Http};
use rocket::serde::json::Json;
use rocket::{response::content, State};
use std::sync::Arc;
use std::{error::Error, fmt};

use eth_crawler::{provider::*, server::*};

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(INDEX_HTML)
}

#[get("/current_block")]
async fn current_block(provider: &State<Arc<Provider<Http>>>) -> Json<String> {
    let block_number = get_block_number(provider).await;

    match block_number {
        Ok(current_block) => Json(format!("Last block: {}", current_block)),
        Err(e) => Json(format!("{}", e)),
    }
}

#[get("/balance/<account>/<time>")]
async fn balance(
    account: String,
    time: String,
    provider: &State<Arc<Provider<Http>>>,
) -> Json<String> {
    let balance = get_balance(provider, account, &time).await;

    match balance {
        Ok(balance) => Json(format!("{}", balance)),
        Err(e) => Json(format!("{}", e)),
    }
}

#[get("/transactions/<account>/<starting_block>")]
async fn transactions(
    account: String,
    starting_block: u64,
    provider: &State<Arc<Provider<Http>>>,
) -> Json<String> {
    let address = parse_address(&account);
    let transactions = get_transactions(provider, account, starting_block).await;

    match transactions {
        Ok(transactions) => {
            // Safe to unwrap here since get_transactions() already parsed the address successfully
            Json(transactions_to_html(&transactions, address.unwrap()))
        }
        Err(e) => Json(format!("{}", e)),
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = ServerArgs::parse();
    let port = if let Some(port_no) = args.port {
        port_no
    } else {
        8080
    };

    let figment = rocket::Config::figment().merge(("port", port));
    let provider =
        Arc::new(Provider::<Http>::try_from(RPC_URL).map_err(|_| ServerError::EthClientNotFound)?);

    let _rocket = rocket::custom(figment)
        .mount("/", routes![index, balance, transactions, current_block])
        .manage(provider)
        .launch()
        .await?;

    Ok(())
}

async fn get_block_number(provider: &Arc<Provider<Http>>) -> Result<U64, ServerError> {
    provider
        .get_block_number()
        .await
        .map_err(|_| ServerError::BlockNotFound)
}

async fn get_balance(
    provider: &Arc<Provider<Http>>,
    address: String,
    time: &str,
) -> Result<Balance, ServerError> {
    let time = time.replace("%20", " ");

    let timestamp = NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| ServerError::InvalidTimestamp)?
        .timestamp();

    let address = parse_address(&address)?;

    provider
        .balance_at_timestamp(address, timestamp)
        .await
        .map_err(|_| ServerError::BalanceNotAvailable)
}

async fn get_transactions(
    provider: &Arc<Provider<Http>>,
    address: String,
    starting_block: u64,
) -> Result<Vec<Transaction>, ServerError> {
    let address = parse_address(&address)?;
    let current_block = provider
        .get_block_number()
        .await
        .map_err(|_| ServerError::BlockNotFound)?
        .as_u64();

    provider
        .transations_of_since_upto(address, starting_block, current_block)
        .await
        .map_err(|_| ServerError::BlockNotFound)
}

fn parse_address(address: &str) -> Result<Address, ServerError> {
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
    let time = "2023-03-14T08:55:04:";
    NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S:")
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
