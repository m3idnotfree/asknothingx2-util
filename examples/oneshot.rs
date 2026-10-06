use std::time::Duration;

use asknothingx2_util::oauth::oneshot::{self, Config, Error};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::new()
        .with_port(8080)
        .with_callback_path("/auth/callback")
        .with_duration(Duration::from_secs(60));

    match oneshot::listen::<Callback>(config).await {
        Ok(callback) => {
            println!("Code: {}", callback.code);
            println!("State: {}", callback.state);
        }
        Err(Error::Timeout) => eprintln!("Timeout"),
        Err(Error::InvalidQuery { query, .. }) => eprintln!("Query: {query:?}"),
        Err(Error::UnexpectedMethod { method }) => eprintln!("Method: {method:?}"),
        Err(Error::UnexpectedPath { expected, actual }) => {
            eprintln!("Expected: {expected:?}");
            eprintln!("Received: {actual:?}");
        }
        Err(e) => eprintln!("{e}"),
    }
}

#[derive(Debug, Deserialize)]
struct Callback {
    pub code: String,
    pub state: String,
}
