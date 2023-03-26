# eth_crawler
Application that allows a user to view transaction data from the Ethereum blockchain associated with a specific wallet address.
Data that can be retrieved are:
 - Transaction history of a wallet starting with specific block
 - Wallet balance at a given time

## Quickstart
Requires stable rustup toolchain. 


#### Web server

    cargo run --bin server -- -p [port_number]
    
If -p parameter is ommited, default port is 8080. After this, a web server should be accessible at http://localhost:[port_number]


#### Command line interface

    cargo run --bin cli -- [wallet_address] -t [time]
    
Gives a wallet balance at a givent time. Time should be in format: "YYYY-MM-DD hh:mm:ss +OOOO", where OOOO is UTC offset.  
To get trancaction history starting from given block, run:

    cargo run --bin cli -- [wallet_address] -s [block_number]

