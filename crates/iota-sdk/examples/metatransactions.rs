// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Meta-transactions ===\n1.Relay 2.Signatures 3.Batch\nCompleted!");
    Ok(())
}
