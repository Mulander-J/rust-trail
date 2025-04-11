use rmcp::ServiceExt;
use std::env;
use tokio::io::{stdin, stdout};

use tools::api::CMCAPI;

mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("COINMARKETCAP_API_KEY")
        .expect("COINMARKETCAP_API_KEY environment variable is not set");

    if api_key.trim().is_empty() {
        panic!("COINMARKETCAP_API_KEY environment variable is empty");
    }

    let server = CMCAPI::new(api_key).serve((stdin(), stdout())).await?;
    server.waiting().await?;

    Ok(())
}
