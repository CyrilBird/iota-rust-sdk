// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_graphql_client::{Client, error::Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new_devnet();

    let system_state = client.latest_system_state().await?;
    println!(
        "Latest system state: {}",
        serde_json::to_string_pretty(&system_state.value_as_json).unwrap()
    );

    Ok(())
}
