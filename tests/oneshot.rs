use std::time::Duration;

use asknothingx2_util::oauth::oneshot::{self, Config, Error};
use serde::Deserialize;
use tokio::time;

#[derive(Debug, Deserialize, PartialEq)]
struct Callback {
    code: String,
    state: String,
}

#[tokio::test]
async fn successful_callback() {
    let config = Config::new()
        .with_port(3001)
        .with_duration(Duration::from_secs(5));

    let server_handle = tokio::spawn(oneshot::listen::<Callback>(config));

    time::sleep(Duration::from_millis(100)).await;

    let resp = reqwest::get("http://127.0.0.1:3001/?code=test_code&state=test_state")
        .await
        .unwrap();

    assert_eq!(200, resp.status());
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(
        "Authorization successful! You can close this window.",
        json["message"]
    );

    let result = server_handle.await.unwrap();
    assert!(result.is_ok());

    let callback = result.unwrap();
    assert_eq!("test_code", callback.code);
    assert_eq!("test_state", callback.state);
}

#[tokio::test]
async fn timeout() {
    let config = Config::new().with_duration(Duration::from_secs(1));

    let result = oneshot::listen::<Callback>(config).await;
    assert!(matches!(result, Err(Error::Timeout)));
}

#[tokio::test]
async fn invalid_query() {
    let config = Config::new()
        .with_port(3002)
        .with_duration(Duration::from_secs(5));

    let server_handle = tokio::spawn(oneshot::listen::<Callback>(config));

    time::sleep(Duration::from_millis(100)).await;

    let resp = reqwest::get("http://127.0.0.1:3002/?code=test_code")
        .await
        .unwrap();

    assert_eq!(400, resp.status());

    let result = server_handle.await.unwrap();

    let Err(Error::InvalidQuery { query, .. }) = result else {
        panic!("expected Error::InvalidQuery");
    };

    assert_eq!("code=test_code", query);
}

#[tokio::test]
async fn unexpected_method() {
    let config = Config::new()
        .with_port(3003)
        .with_duration(Duration::from_secs(5));

    let server_handle = tokio::spawn(oneshot::listen::<Callback>(config));

    time::sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post("http://127.0.0.1:3003/?code=test_code&state=test_state")
        .send()
        .await
        .unwrap();

    assert_eq!(405, resp.status());

    let result = server_handle.await.unwrap();

    let Err(Error::UnexpectedMethod { method }) = result else {
        panic!("expected Error::UnexpectedMethod");
    };

    assert_eq!("POST", method);
}

#[tokio::test]
async fn unexpected_path() {
    let config = Config::new()
        .with_port(3004)
        .with_callback_path("/callback")
        .with_duration(Duration::from_secs(5));

    let server_handle = tokio::spawn(oneshot::listen::<Callback>(config));

    time::sleep(Duration::from_millis(100)).await;

    let resp = reqwest::get("http://127.0.0.1:3004/wrong?code=test_code&state=test_state")
        .await
        .unwrap();

    assert_eq!(404, resp.status());

    let result = server_handle.await.unwrap();

    let Err(Error::UnexpectedPath { expected, actual }) = result else {
        panic!("expected Error::UnexpectedPath");
    };

    assert_eq!("/callback", expected);
    assert_eq!("/wrong", actual);
}

#[tokio::test]
async fn custom_message() {
    let config = Config::new()
        .with_port(3005)
        .with_message("Custom success message!")
        .with_duration(Duration::from_secs(5));

    let server_handle = tokio::spawn(oneshot::listen::<Callback>(config));

    time::sleep(Duration::from_millis(100)).await;

    let resp = reqwest::get("http://127.0.0.1:3005/?code=test_code&state=test_state")
        .await
        .unwrap();

    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!("Custom success message!", json["message"]);

    let result = server_handle.await.unwrap();
    assert!(result.is_ok());
}
