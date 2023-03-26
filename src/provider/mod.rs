pub mod balance;
pub mod transactions;

pub use balance::{Balance, EthCrawlerBalance};
use core::{future::Future, marker::Send, pin::Pin};
use ethers::prelude::*;
pub use transactions::{direction_address, EthCrawlerTransactions};

pub const RETRY_COUNT: usize = 4;
pub const RPC_URL: &str = "https://eth.llamarpc.com";

/// Retries function provided as an argument if it failed due to a network error.
/// Nuber of attempts is limmited by RETRY_COUNT
pub async fn repeat_if_network_error<'a, R, P: JsonRpcClient, T>(
    f: &impl Fn(&'a Provider<P>, U64) -> Pin<Box<T>>,
    provider: &'a Provider<P>,
    arg: U64,
) -> Result<R, ProviderError>
where
    T: Future<Output = Result<R, ProviderError>> + Send + ?Sized,
{
    let mut res = f(provider, arg).await;

    let mut attempt = 0;
    while let Err(ProviderError::JsonRpcClientError(_)) = res {
        if attempt == RETRY_COUNT {
            break;
        }
        attempt += 1;
        res = f(provider, arg).await;
    }
    res
}
