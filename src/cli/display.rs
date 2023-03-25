use crate::{balance::Balance, transactions::direction_address};
use ethers::prelude::*;

pub fn display_header() {
    println!(
        "\n| {:15} | {:10} | {:20} | {:20} |\n{:-<5$}",
        "Tx hash", "Block", "To/from", "Value (ETH)", "", 78
    )
}
pub fn print_formated(transaction: Transaction, target: Address) {
    let (direction, address) = direction_address(&transaction, target);

    println!(
        "| {:<15} | {:<10} | {:4} {:<15} | {:<20} |",
        transaction.hash.to_string(),
        transaction
            .block_number
            .map(|block_number| block_number.to_string())
            .unwrap_or_else(|| "Pending".into()),
        direction,
        address,
        Balance::from(transaction.value)
    );
}
