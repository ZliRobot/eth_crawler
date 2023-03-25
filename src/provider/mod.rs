pub mod balance;
pub mod transactions;

pub use balance::{Balance, EthCrawlerBalance};
pub use transactions::{direction_address, EthCrawlerTransactions};

pub const RETRY_COUNT: usize = 4;
pub const RPC_URL: &str = "https://eth.llamarpc.com";
