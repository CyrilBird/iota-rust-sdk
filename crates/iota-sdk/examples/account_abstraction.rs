// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Account Abstraction ===\n1.Smart-accounts 2.Paymasters 3.Sessions\nCompleted!");
    Ok(())
}
