pub mod display;
pub use display::{transactions_to_html, INDEX_HTML};

use clap::Parser;

#[derive(Parser)]
pub struct ServerArgs {
    /// Port number to listen on. (Default: 8080)
    #[arg(short, long)]
    pub port: Option<u64>,
}
