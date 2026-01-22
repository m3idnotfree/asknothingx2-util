use std::time::Duration;

use asknothingx2_util::oauth::signed_token::{
    current_timestamp, extract_datetime, extract_timestamp, generate, generate_secret_key,
    is_expired, token_age, verify, verify_with_config, TokenConfig, TokenError,
};
use chrono::Utc;

#[tokio::test]
async fn basic_validation() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, Some("user123"));

    assert!(verify(&secret_key, &token, Some("user123"), 3600).is_ok());
    assert!(verify(&secret_key, &token, Some("user456"), 3600).is_err());

    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(verify(&secret_key, &token, Some("user123"), 0).is_err());
}

#[tokio::test]
async fn clock_skew_tolerance() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, Some("user123"));

    assert!(verify_with_config(
        &secret_key,
        &token,
        Some("user123"),
        &TokenConfig::new(3, 3600)
    )
    .is_ok());
    assert!(verify_with_config(
        &secret_key,
        &token,
        Some("user456"),
        &TokenConfig::new(3, 3600)
    )
    .is_err());

    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(verify_with_config(
        &secret_key,
        &token,
        Some("user123"),
        &TokenConfig::new(3, 0)
    )
    .is_ok());
}

#[test]
fn test_extract_timestamp() {
    let current_time = current_timestamp();
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, None);

    let extracted = extract_timestamp(&token);
    assert_eq!(current_time, extracted.unwrap());
}

#[test]
fn verify_token_with_errors() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, Some("user123"));

    assert!(verify_with_config(
        &secret_key,
        &token,
        Some("user123"),
        &TokenConfig::default()
    )
    .is_ok());

    assert_eq!(
        verify_with_config(
            &secret_key,
            &token,
            Some("invalid"),
            &TokenConfig::default()
        ),
        Err(TokenError::InvalidSignature)
    );

    assert_eq!(
        verify_with_config(
            &secret_key,
            &token,
            Some("user123"),
            &TokenConfig::new(0, 0)
        ),
        Err(TokenError::Expired)
    );

    assert_eq!(
        verify_with_config(
            &secret_key,
            "invalid_token",
            Some("user123"),
            &TokenConfig::default()
        ),
        Err(TokenError::InvalidFormat)
    );
}

#[test]
fn token_without_context() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, None);

    assert!(verify(&secret_key, &token, None, 3600).is_ok());
    assert!(verify(&secret_key, &token, Some("user123"), 3600).is_err());
}

#[test]
fn test_token_age() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, None);

    let age = token_age(&token).unwrap();
    assert!((0..2).contains(&age));
}

#[test]
fn test_is_expired() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, None);

    assert_eq!(is_expired(&token, 3600), Ok(false));
    assert_eq!(is_expired(&token, 0), Ok(true));
    assert_eq!(is_expired("invalid", 3600), Err(TokenError::InvalidFormat));
}

#[test]
fn test_extract_datetime() {
    let secret_key = generate_secret_key();
    let token = generate(&secret_key, None);

    let dt = extract_datetime(&token).unwrap();
    let now = Utc::now();

    assert!((now.timestamp() - dt.timestamp()).abs() < 2);
}

#[test]
fn config_builder() {
    let config = TokenConfig::new(0, 1800);
    assert_eq!(config.clock_skew, None);
}
