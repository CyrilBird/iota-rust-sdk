// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Network Effects ===\n1.Adoption 2.Utility 3.Growth\nCompleted!");
    Ok(())
}
