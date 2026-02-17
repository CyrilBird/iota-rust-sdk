// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Privacy ===\n1.ZK-proofs 2.Mixers 3.Anonymous-credentials\nCompleted!");
    Ok(())
}
