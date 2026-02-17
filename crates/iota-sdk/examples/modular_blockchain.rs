// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Modular ===\n1.Execution 2.Data-availability 3.Consensus\nCompleted!");
    Ok(())
}
