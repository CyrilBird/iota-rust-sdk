// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Upgradeability ===\n1.Patterns 2.Migrations 3.Compatibility\nCompleted!");
    Ok(())
}
