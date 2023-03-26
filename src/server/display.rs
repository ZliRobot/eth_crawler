use crate::provider::*;
use ethers::prelude::*;
use std::fmt::Write;

pub fn transactions_to_html(transactions: &[Transaction], target: Address) -> String {
    let mut html = String::new();

    html.push_str(TABLE_HEADER);

    for transaction in transactions {
        let (direction, address) = direction_address(transaction, target);

        _ = write!(
            html,
            "
            <tr>\n
                <th><a href=https://etherscan.io/tx/{tx:?}>{tx}</a></th>\n
                <th><a href=https://etherscan.io/block/{block}>{block}</a></th>\n
                <th>{direction} <a href=https://etherscan.io/address/{address}>{address}</a></th>\n
                <th>{balance}</th>\n
            </tr>\n
            ",
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

static TABLE_HEADER: &str = "
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
        <th>Amount (ETH)</th>\n
    </tr>\n";
