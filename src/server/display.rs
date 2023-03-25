use crate::provider::*;
use ethers::prelude::*;
use std::fmt::Write;

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
        let (direction, address) = direction_address(transaction, target);

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

pub static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
<body onload="setInterval(print_current_block, 5000)">
<style>
*{ cursor: inherit;}
</style>

<h2>Account info</h2>
<p id="current_block"> Last block: </p>

<label for="address">Account address:</label>
<input type="text" id="address" name="address" size="50"><br>
  

<label for="datetime"> Balance at: </label>
<input type="datetime-local" id="datetime" name="datetime" step=1> UTC:  
<label id=balance> -- </label> ETH<br>

<label for="starting_block"> Transactions starting from block: </label>
<input type="number" id="starting_block" name="starting_block"><br>

<button onclick="submit()">Submit</button>

<p id="transactions"></p>

<script>
async function print_current_block() {
    document.getElementById("current_block").innerHTML = await fetch(window.location.pathname + "current_block").then((res) => res.json());
}

async function submit() {
    document.body.style.cursor = 'wait';

    const address = document.getElementById("address").value;
    const datetime = document.getElementById("datetime").value;
    const starting_block = document.getElementById("starting_block").valueAsNumber;

    if  (datetime != "") {
        document.getElementById("balance").innerHTML = await fetch(window.location.pathname + "balance/ " + address + "/" + datetime).then((res) => res.json());
    }

    if (!isNaN(starting_block)) {
        document.getElementById("transactions").innerHTML = await fetch(window.location.pathname + "transactions/" + address + "/" + starting_block).then((res) => res.json());
    }

    document.body.style.cursor = 'default';
}
</script>

</body>
</html>"#;
