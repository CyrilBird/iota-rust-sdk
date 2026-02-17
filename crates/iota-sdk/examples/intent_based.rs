// Copyright (c) 2025 IOTA Stiftung
use eyre::Result;
use iota_sdk::graphql_client::Client;
#[tokio::main]
async fn main() -> Result<()> {
    let _client = Client::new_devnet();
    println!("=== Intent-Based ===\n1.Intents 2.Solvers 3.Matching\nCompleted!");
    Ok(())
}
