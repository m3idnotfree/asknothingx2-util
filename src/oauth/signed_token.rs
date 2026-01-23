//! Time-based HMAC-signed token generation and validation.
//!
//! This module provides a secure way to generate and validate time-stamped tokens
//! using HMAC-SHA256 signatures. It's commonly used for CSRF protection, state tokens,
//! and other security-sensitive operations requiring time-limited authentication.
//!
//! # Features
//!
//! - **Time-based validation**: Tokens include timestamps and can be validated with configurable max age
//! - **Clock skew tolerance**: Handle slight time differences between systems
//! - **Context binding**: Optionally bind tokens to specific contexts (e.g., user IDs)
//! - **Cryptographically secure**: Uses HMAC-SHA256 for signature generation
//! - **URL-safe encoding**: Tokens are base64url-encoded without padding
//!
//! # Token Format
//!
//! Tokens are structured as: `base64url(timestamp:signature)`
//! - `timestamp`: Unix timestamp in seconds
//! - `signature`: HMAC-SHA256 signature of `timestamp:context`
//!
//! # Examples
//!
//! ```
//! # use asknothingx2_util::oauth::signed_token;
//! let secret = signed_token::generate_secret_key();
//! let token = signed_token::generate(&secret, Some("user123"));
//!
//! // Validate within 1 hour
//! assert!(signed_token::verify(&secret, &token, Some("user123"), 3600).is_ok());
//! ```
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, TimeZone, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Configuration for token validation behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenConfig {
    pub clock_skew: Option<u64>,
    pub max_age: u64,
}

impl Default for TokenConfig {
    fn default() -> Self {
        Self {
            clock_skew: None,
            // 30 minutes
            max_age: 1800,
        }
    }
}

impl TokenConfig {
    /// Creates a new token configuration.
    ///
    /// # Arguments
    ///
    /// * `clock_skew` - Clock skew tolerance in seconds (0 means disabled)
    /// * `max_age` - Maximum token age in seconds
    #[inline]
    pub fn new(clock_skew: u64, max_age: u64) -> Self {
        Self {
            clock_skew: (clock_skew > 0).then_some(clock_skew),
            max_age,
        }
    }

    /// Sets the clock skew tolerance.
    ///
    /// # Arguments
    ///
    /// * `clock_skew` - Tolerance in seconds (0 to disable)
    #[inline]
    pub fn with_clock_skew(mut self, clock_skew: u64) -> Self {
        self.clock_skew = (clock_skew > 0).then_some(clock_skew);
        self
    }

    /// Sets the maximum token age.
    #[inline]
    pub fn with_max_age(mut self, max_age: u64) -> Self {
        self.max_age = max_age;
        self
    }
}

/// Generates a new time-stamped signed token.
///
/// Creates a token containing the current timestamp and an optional context string,
/// signed with HMAC-SHA256 using the provided secret key.
///
/// # Arguments
///
/// * `secret_key` - 32-byte secret key for HMAC signing
/// * `context` - Optional context string to bind the token to (e.g., user ID, session ID)
///
/// # Returns
///
/// A base64url-encoded token string without padding.
///
/// # Examples
///
/// ```
/// # use asknothingx2_util::oauth::signed_token::{generate, generate_secret_key};
/// let secret = generate_secret_key();
///
/// let token = generate(&secret, Some("user123"));
/// ```
pub fn generate(secret_key: &[u8; 32], context: Option<&str>) -> String {
    generate_at_time(secret_key, context, current_timestamp())
}

/// Generates a token with a specific timestamp.
#[inline]
pub fn generate_at_time(secret_key: &[u8; 32], context: Option<&str>, timestamp: i64) -> String {
    let context_str = context.unwrap_or("");
    let payload = format!("{timestamp}:{context_str}");

    let mut mac = HmacSha256::new_from_slice(secret_key).expect("HMAC can accept keys of any size");
    mac.update(payload.as_bytes());
    let signature = mac.finalize().into_bytes();

    let token_data = format!("{timestamp}:{}", hex::encode(signature));
    URL_SAFE_NO_PAD.encode(token_data.as_bytes())
}

/// Validates a token.
///
/// # Arguments
///
/// * `secret_key` - 32-byte secret key used to generate the token
/// * `token` - Token string to validate
/// * `context` - Expected context (must match the one used during generation)
/// * `max_age_seconds` - Maximum age in seconds
///
#[inline]
pub fn verify(
    secret_key: &[u8; 32],
    token: &str,
    context: Option<&str>,
    max_age_seconds: u64,
) -> Result<(), TokenError> {
    verify_with_config(
        secret_key,
        token,
        context,
        &TokenConfig::default().with_max_age(max_age_seconds),
    )
}

/// Validates a token with a custom configuration.
#[inline]
pub fn verify_with_config(
    secret_key: &[u8; 32],
    token: &str,
    context: Option<&str>,
    config: &TokenConfig,
) -> Result<(), TokenError> {
    verify_at_time(secret_key, token, context, current_timestamp(), config)
}

/// Validates a token at a specific validation time.
pub fn verify_at_time(
    secret_key: &[u8; 32],
    token: &str,
    context: Option<&str>,
    validation_time: i64,
    config: &TokenConfig,
) -> Result<(), TokenError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(token)
        .map_err(|_| TokenError::InvalidFormat)?;

    let decoded_str = String::from_utf8(decoded).map_err(|_| TokenError::InvalidFormat)?;

    let mut parts = decoded_str.splitn(2, ':');
    let timestamp_str = parts.next().ok_or(TokenError::InvalidFormat)?;
    let signature_str = parts.next().ok_or(TokenError::InvalidFormat)?;

    let timestamp: i64 = timestamp_str
        .parse()
        .map_err(|_| TokenError::InvalidFormat)?;

    if timestamp < 0 {
        return Err(TokenError::InvalidTimestamp);
    }

    let age = validation_time - timestamp;
    let tolerance = config.clock_skew.unwrap_or(0) as i64;
    let max_age = config.max_age as i64;
    let effective_max_age = max_age + tolerance;
    let min_age = -tolerance;

    if age >= effective_max_age {
        return Err(TokenError::Expired);
    }
    if age < min_age {
        return Err(TokenError::InvalidTimestamp);
    }

    let provided_signature = hex::decode(signature_str).map_err(|_| TokenError::InvalidFormat)?;

    let context_str = context.unwrap_or("");
    let payload = format!("{timestamp}:{context_str}");

    let mut mac = HmacSha256::new_from_slice(secret_key).expect("HMAC can accept keys of any size");
    mac.update(payload.as_bytes());

    mac.verify_slice(&provided_signature)
        .map_err(|_| TokenError::InvalidSignature)
}

/// Checks if a token is expired based on max age.
///
/// # Errors
///
/// Returns [`TokenError::InvalidFormat`] if the token cannot be decoded or parsed.
/// Returns [`TokenError::InvalidTimestamp`] if the timestamp is negative or represents a future time.
pub fn is_expired(token: &str, max_age_seconds: u64) -> Result<bool, TokenError> {
    let timestamp = extract_timestamp(token)?;

    if timestamp < 0 {
        return Err(TokenError::InvalidTimestamp);
    }

    let now = current_timestamp();
    let age = now - timestamp;

    if age < 0 {
        return Err(TokenError::InvalidTimestamp);
    }

    Ok(age >= max_age_seconds as i64)
}

/// Extracts the creation timestamp from a token as a [`DateTime<Utc>`].
///
/// # Errors
///
/// Returns [`TokenError::InvalidFormat`] if the token cannot be decoded or parsed.
/// Returns [`TokenError::InvalidTimestamp`] if the timestamp cannot be converted to a valid DateTime.
#[inline]
pub fn extract_datetime(token: &str) -> Result<DateTime<Utc>, TokenError> {
    let timestamp = extract_timestamp(token)?;
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .ok_or(TokenError::InvalidTimestamp)
}

/// Calculates the age of a token in seconds to current time.
///
/// # Errors
///
/// Returns [`TokenError::InvalidFormat`] if the token cannot be decoded or parsed.
#[inline]
pub fn token_age(token: &str) -> Result<i64, TokenError> {
    let timestamp = extract_timestamp(token)?;
    Ok(current_timestamp() - timestamp)
}

/// Extracts the Unix timestamp from a token.
///
/// # Errors
///
/// Returns [`TokenError::InvalidFormat`] if the token cannot be decoded,
/// is not valid UTF-8, or doesn't contain a valid timestamp.
#[inline]
pub fn extract_timestamp(token: &str) -> Result<i64, TokenError> {
    let decoded = URL_SAFE_NO_PAD
        .decode(token)
        .map_err(|_| TokenError::InvalidFormat)?;
    let decoded_str = String::from_utf8(decoded).map_err(|_| TokenError::InvalidFormat)?;
    let timestamp_str = decoded_str
        .split(':')
        .next()
        .ok_or(TokenError::InvalidFormat)?;
    timestamp_str.parse().map_err(|_| TokenError::InvalidFormat)
}

/// Generates a random 32-byte secret key.
#[inline]
pub fn generate_secret_key() -> [u8; 32] {
    rand::random()
}

/// Returns the current Unix timestamp in seconds.
#[inline]
pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum TokenError {
    #[error("token format is invalid")]
    InvalidFormat,
    #[error("token signature is invalid")]
    InvalidSignature,
    #[error("token has expired")]
    Expired,
    #[error("token timestamp is invalid")]
    InvalidTimestamp,
}
