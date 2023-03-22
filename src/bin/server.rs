// use chrono::DateTime;
use clap::Parser;
// use ethers::prelude::*;
// use std::sync::Arc;

use eth_crawler::modules::*;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(())
}
