use super::balance::Balance;
use ethers::prelude::*;
use std::fmt::Write;

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

pub fn transactions_to_html(transactions: &[Transaction], target: Address) -> String {
    let mut html = String::new();

    html.push_str(
        "
    <style>\n
        table {\n
            font-family: arial, sans-serif;\n
            border-collapse: collapse;\n
            width: 100%;\n
        }\n
    \n
        td, th {\n
            border: 1px solid #dddddd;\n
            text-align: left;\n
            padding: 8px;\n
        }\n
    \n
        tr:nth-child(even) {\n
            background-color: #dddddd;\n
        }\n
    </style>\n
    <table>\n
        <tr>\n
            <th>Transaction hash</th>\n
            <th>Block</th>\n
            <th>To/From</th>\n
            <th>Amount</th>\n
        </tr>\n",
    );

    for transaction in transactions {
        let (direction, address) = direction_address(&transaction, target);

        _ = write!(
            html,
            "
        <tr>\n
            <th><a href=https://etherscan.io/tx/{tx:?}>{tx}</a></th>\n
            <th><a href=https://etherscan.io/block/{block}>{block}</a></th>\n
            <th>{direction} <a href=https://etherscan.io/address/{address}>{address}</a></th>\n
            <th>{balance}</th>\n
        </tr>\n",
            tx = transaction.hash,
            block = transaction
                .block_number
                .map(|block_number| block_number.to_string())
                .unwrap_or_else(|| "Pending".into()),
            balance = Balance::from(transaction.value)
        );
    }

    html.push_str("</table>\n");
    html
}

fn direction_address(transaction: &Transaction, target: Address) -> (String, String) {
    if transaction.from == target {
        (
            "to".to_string(),
            transaction
                .to
                .map(|to| format!("{:?}", to))
                .unwrap_or_else(|| "Creating SC".into()),
        )
    } else {
        ("from".into(), format!("{:?}", transaction.from))
    }
}
