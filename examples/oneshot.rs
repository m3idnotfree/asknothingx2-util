use std::time::Duration;

use asknothingx2_util::oauth::oneshot::{self, Config};
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
        Err(e) if e.is_timeout() => {
            eprintln!("Timeout")
        }
        Err(e) if e.is_invalid_query() => {
            eprintln!("Query: {}", e.query().unwrap());
        }
        Err(e) if e.is_unexpected_method() => {
            eprintln!("Method: {}", e.method().unwrap());
        }
        Err(e) if e.is_unexpected_path() => {
            let (expected, actual) = e.path().unwrap();
            eprintln!("Expected: {}", expected);
            eprintln!("Received: {}", actual);
        }
        Err(e) if e.is_shutdown() => {}
        Err(_e) => {}
    }
}

#[derive(Debug, Deserialize)]
struct Callback {
    pub code: String,
    pub state: String,
}
