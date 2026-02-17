// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== L2 Scaling ===\n1.Rollups 2.Validium 3.Volition\nCompleted!");
    Ok(())
}
