use clap::Parser;
use ethers::{prelude::*, providers::Http};
use rocket::serde::json::Json;
use rocket::{fs::NamedFile, State};
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

use eth_crawler::{provider::*, server::*};

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(PathBuf::new().join("src").join("server").join("index.html"))
        .await
        .ok()
}

#[get("/current_block")]
async fn current_block(provider: &State<Arc<Provider<Http>>>) -> Json<String> {
    let block_number = get_last_block_number(provider).await;

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
    let address = match parse_address(&account) {
        Ok(address) => address,
        Err(_) => return Json("Invalid address!".to_string()),
    };

    let transactions = get_transactions(provider, address, starting_block).await;

    match transactions {
        Ok(transactions) => Json(transactions_to_html(&transactions, address)),
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
