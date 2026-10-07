//! Oneshot OAuth callback server for development and testing.
//!
//! A lightweight HTTP server that listens for a single OAuth callback and automatically
//! shuts down. Useful for testing OAuth flows from various providers on localhost.
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use asknothingx2_util::oauth::oneshot::{self, Config, Error};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Callback {
//!     pub code: String,
//!     pub state: String,
//! }
//!
//! async fn callback() -> Result<Callback, Error> {
//!     let config = Config::new()
//!         .with_port(8080)
//!         .with_callback_path("/auth/callback")
//!         .with_duration(Duration::from_secs(10));
//!
//!     oneshot::listen(config).await
//! }
//! ```
//!
//! # Error Handling
//! ```
//! # use asknothingx2_util::oauth::oneshot::{self, Config, Error};
//! # async fn run(config:Config) {
//! match oneshot::listen(config).await {
//!     Ok(callback) => { callback },
//!     Err(Error::Timeout) => eprintln!("Timeout"),
//!     Err(Error::InvalidQuery { query, .. }) => eprintln!("Query: {query:?}"),
//!     Err(Error::UnexpectedMethod { method }) => eprintln!("Method: {method:?}"),
//!     Err(Error::UnexpectedPath { expected, actual }) => {
//!         eprintln!("Expected: {expected:?}");
//!         eprintln!("Received: {actual:?}");
//!     }
//!     Err(e) => eprintln!("{e}"),
//! }
//! # }
//! ```
use std::{
    convert::Infallible,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use hyper::{
    Method, Request, Response, StatusCode,
    body::Incoming,
    header::{CONTENT_TYPE, HeaderValue},
    server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle, time::sleep};
use tracing::debug;

/// Configuration for the oneshot OAuth callback server.
///
/// # Defaults
///
/// - **port**: `3000`
/// - **path**: `"/"`
/// - **duration**: `30 seconds`
/// - **message**: `"Authorization successful! You can close this window."`
pub struct Config {
    port: u16,
    path: String,
    duration: Duration,
    message: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 3000,
            path: "/".to_string(),
            duration: Duration::from_secs(30),
            message: "Authorization successful! You can close this window.".to_string(),
        }
    }

    pub fn with_callback_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// The server will return [`Error::Timeout`] if no callback is received
    /// within this duration.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }
}

/// Starts a oneshot HTTP server and waits for an OAuth callback.
///
/// This function binds to `127.0.0.1` on the configured port and listens for a single
/// HTTP GET request. When a valid callback is received, the query parameters are parsed
/// into type `T` and the server automatically shuts down.
///
/// # Type Parameters
///
/// * `T` - The callback parameter type that implements [`serde::Deserialize`].
///
/// # Server Behavior
///
/// The server shuts down immediately when:
/// - O - A valid callback is received (returns `Ok(T)`)
/// - X - Query parsing fails (returns Err([Error::InvalidQuery]))
/// - X - Wrong HTTP method is used (returns Err([Error::UnexpectedMethod]))
/// - X - Wrong path is requested (returns Err([Error::UnexpectedPath]))
/// - X - Timeout is reached (returns Err([Error::Timeout]))
/// - X - Ctrl+C is pressed (returns Err([Error::Shutdown]))
pub async fn listen<T>(config: Config) -> Result<T, Error>
where
    T: DeserializeOwned + Send + 'static,
{
    let (tx, rx) = oneshot::channel::<Result<T, Error>>();

    let state = Arc::new(AppState {
        tx: Mutex::new(Some(tx)),
        path: config.path,
        message: config.message,
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    debug!(%addr, "starting OAuth callback server");

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| Error::BindFailed {
            addr: addr.to_string(),
            source: e,
        })?;

    let server_handle: JoinHandle<IoError> = tokio::spawn(async move {
        loop {
            let (stream, remote_addr) = match listener.accept().await {
                Ok(accepted) => accepted,
                Err(e) => return e,
            };
            debug!(%remote_addr, "accepted connection");

            let io = TokioIo::new(stream);
            let state = state.clone();

            tokio::spawn(async move {
                let service = service_fn(|req| handle_request::<T>(req, state.clone()));

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    debug!(error = %err, "failed to serve connection");
                }
            });
        }
    });

    tokio::select! {
        result = rx => {
            debug!("stopping OAuth callback server");
            match result {
                Ok(result) => {
                    server_handle.abort();
                    result
                }
                Err(_) => {
                    let result = server_handle.await;
                    Err(Error::Io(result.unwrap_or_else(IoError::from)))
                }
            }
        }
        _ = sleep(config.duration) => {
            server_handle.abort();
            Err(Error::Timeout)
        }
        _ = tokio::signal::ctrl_c() => {
            server_handle.abort();
            Err(Error::Shutdown)
        }
    }
}
struct AppState<T> {
    tx: Mutex<Option<oneshot::Sender<Result<T, Error>>>>,
    path: String,
    message: String,
}

async fn handle_request<T>(
    req: Request<Incoming>,
    state: Arc<AppState<T>>,
) -> Result<Response<String>, Infallible>
where
    T: DeserializeOwned + Send + 'static,
{
    let method = req.method();
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    debug!(%method, path, query, "received request");

    if method != Method::GET {
        if let Some(sender) = state.tx.lock().unwrap().take() {
            let _ = sender.send(Err(Error::UnexpectedMethod {
                method: method.clone(),
            }));
        }

        return Ok(json_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "error",
            "Method not allowed",
        ));
    }

    if path != state.path {
        if let Some(sender) = state.tx.lock().unwrap().take() {
            let _ = sender.send(Err(Error::UnexpectedPath {
                expected: state.path.to_string(),
                actual: path.to_string(),
            }));
        }

        return Ok(json_response(StatusCode::NOT_FOUND, "error", "Not found"));
    }

    let params: T = match serde_urlencoded::from_str(query) {
        Ok(p) => p,
        Err(e) => {
            let error_msg = e.to_string();
            if let Some(sender) = state.tx.lock().unwrap().take() {
                let _ = sender.send(Err(Error::InvalidQuery {
                    query: query.to_string(),
                    source: e,
                }));
            }

            return Ok(json_response(StatusCode::BAD_REQUEST, "error", &error_msg));
        }
    };

    if let Some(sender) = state.tx.lock().unwrap().take() {
        let _ = sender.send(Ok(params));
    }

    Ok(json_response(StatusCode::OK, "message", &state.message))
}

fn json_response(status: StatusCode, key: &str, message: &str) -> Response<String> {
    let body = serde_json::json!({ key: message }).to_string();
    let mut response = Response::new(body);
    *response.status_mut() = status;
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    response
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to bind to address {:?}", addr)]
    BindFailed {
        addr: String,
        #[source]
        source: IoError,
    },
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("invalid OAuth callback query")]
    InvalidQuery {
        query: String,
        #[source]
        source: serde_urlencoded::de::Error,
    },
    #[error("unexpected HTTP method {:?}", method)]
    UnexpectedMethod { method: Method },
    #[error("unexpected callback path {:?}", actual)]
    UnexpectedPath { expected: String, actual: String },
    #[error("shutdown signal received")]
    Shutdown,
    #[error("timeout waiting for OAuth callback")]
    Timeout,
}
