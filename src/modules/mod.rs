pub mod arguments;
pub mod balance;
pub mod display;
pub mod transactions;

pub use arguments::{CliArgs, ServerArgs};
pub use balance::{Balance, EthCrawlerBalance};
pub use display::{display_header, print_formated};
pub use transactions::EthCrawlerTransactions;

pub const RETRY_COUNT: usize = 4;
pub const RPC_URL: &str = "https://eth.llamarpc.com";
