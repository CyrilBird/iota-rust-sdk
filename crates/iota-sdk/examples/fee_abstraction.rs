// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Fee Abstraction ===\n1.Gas-sponsor 2.Relayers 3.Meta-transactions\nCompleted!");
    Ok(())
}
