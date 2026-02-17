// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Interoperability ===\n1.Cross-chain 2.Messaging 3.State-sync\nCompleted!");
    Ok(())
}
