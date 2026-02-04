//! Oneshot OAuth callback server for development and testing.
//!
//! A lightweight HTTP server that listens for a single OAuth callback and automatically
//! shuts down. Useful for testing OAuth flows from various providers on localhost.
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use asknothingx2_util::oauth::oneshot::{self, Config, ServerError};
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Callback {
//!     pub code: String,
//!     pub state: String,
//! }
//!
//! async fn callback() -> Result<Callback, ServerError> {
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
//! # use asknothingx2_util::oauth::oneshot::{self, Config};
//! # async fn run(config:Config) {
//! match oneshot::listen(config).await {
//!     Ok(callback) => { callback }
//!     Err(e) => {
//!         if e.is_timeout() {
//!             eprintln!("Timeout");
//!         } else if e.is_invalid_query() {
//!             eprintln!("Query: {}", e.query().unwrap());
//!         } else if e.is_unexpected_path() {
//!             let (expected, actual) = e.path().unwrap();
//!             eprintln!("Expected: {}", expected);
//!             eprintln!("Received: {}", actual);
//!         } else if e.is_shutdown() {
//!             eprintln!("Shutdown");
//!         }
//!     }
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

use http_body_util::Full;
use hyper::{
    Method, Request, Response, StatusCode,
    body::{Bytes, Incoming},
    server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use serde::{Serialize, de::DeserializeOwned};
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

    /// The server will return [`ServerError::Timeout`] if no callback is received
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
/// - X - Query parsing fails (returns Err([ServerError::InvalidQuery]))
/// - X - Wrong HTTP method is used (returns Err([ServerError::UnexpectedMethod]))
/// - X - Wrong path is requested (returns Err([ServerError::UnexpectedPath]))
/// - X - Timeout is reached (returns Err([ServerError::Timeout]))
/// - X - Ctrl+C is pressed (returns Err([ServerError::Shutdown]))
pub async fn listen<T>(config: Config) -> Result<T, ServerError>
where
    T: DeserializeOwned + Send + 'static,
{
    let (tx, rx) = oneshot::channel::<Result<T, ServerError>>();

    let state = Arc::new(AppState {
        tx: Arc::new(Mutex::new(Some(tx))),
        path: config.path,
        message: config.message,
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    debug!("Starting OAuth callback server on {}", addr);

    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| ServerError::BindFailed {
            addr: addr.to_string(),
            source: e,
        })?;

    let server_handle: JoinHandle<Result<(), ServerError>> = tokio::spawn(async move {
        loop {
            let (stream, remote_addr) = listener.accept().await?;
            debug!("Accepted connection from {}", remote_addr);

            let io = TokioIo::new(stream);
            let state = state.clone();

            tokio::spawn(async move {
                let service = service_fn(|req| handle_request::<T>(req, state.clone()));

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    debug!("Error serving connection: {:?}", err);
                }
            });
        }
    });

    tokio::select! {
        result = rx => {
            debug!("Shutdown OAuth callback server");
            server_handle.abort();
            match result {
                Ok(Ok(callback)) => Ok(callback),
                Ok(Err(e)) => Err(e),
                Err(_) => Err(ServerError::Shutdown),
            }
        }
        _ = sleep(config.duration) => {
            debug!("OAuth callback server timed out");
            server_handle.abort();
            Err(ServerError::Timeout)
        }
        _ = tokio::signal::ctrl_c() => {
            debug!("OAuth callback server received shutdown signal");
            server_handle.abort();
            Err(ServerError::Shutdown)
        }
    }
}
struct AppState<T> {
    #[allow(clippy::type_complexity)]
    tx: Arc<Mutex<Option<oneshot::Sender<Result<T, ServerError>>>>>,
    path: String,
    message: String,
}

#[derive(serde::Serialize)]
struct CallbackResponse {
    message: String,
}

async fn handle_request<T>(
    req: Request<Incoming>,
    state: Arc<AppState<T>>,
) -> Result<Response<Full<Bytes>>, Infallible>
where
    T: DeserializeOwned + Send + 'static,
{
    let method = req.method();
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");

    debug!("Received request: {} {} (query: {})", method, path, query);

    if method != Method::GET {
        debug!("Unexpected HTTP method: expected GET, got {}", method);

        if let Some(sender) = state.tx.lock().unwrap().take() {
            let _ = sender.send(Err(ServerError::UnexpectedMethod {
                method: method.clone(),
            }));
        }

        return Ok(error_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed",
        ));
    }

    if path != state.path {
        debug!("Unexpected path: expected '{}', got '{}'", state.path, path);

        if let Some(sender) = state.tx.lock().unwrap().take() {
            let _ = sender.send(Err(ServerError::UnexpectedPath {
                expected: state.path.to_string(),
                actual: path.to_string(),
            }));
        }

        return Ok(error_response(StatusCode::NOT_FOUND, "Not found"));
    }

    let params: T = match serde_urlencoded::from_str(query) {
        Ok(p) => {
            debug!("Successfully parsed OAuth callback parameters");
            p
        }
        Err(e) => {
            let error_msg = e.to_string();
            debug!("Failed to parse OAuth callback query `{}`: {}", query, e);

            if let Some(sender) = state.tx.lock().unwrap().take() {
                let _ = sender.send(Err(ServerError::InvalidQuery {
                    query: query.to_string(),
                    source: e,
                }));
            }

            return Ok(error_response(StatusCode::BAD_REQUEST, &error_msg));
        }
    };

    if let Some(sender) = state.tx.lock().unwrap().take() {
        let _ = sender.send(Ok(params));
    }

    let response = CallbackResponse {
        message: state.message.clone(),
    };

    Ok(json_response(StatusCode::OK, &response))
}

fn json_response<T: Serialize>(status: StatusCode, body: &T) -> Response<Full<Bytes>> {
    let json = serde_json::to_vec(body).unwrap();
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(json)))
        .unwrap()
}

fn error_response(status: StatusCode, message: &str) -> Response<Full<Bytes>> {
    let error = serde_json::json!({ "error": message });
    json_response(status, &error)
}

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("failed to bind to address `{addr}`: {source}")]
    BindFailed { addr: String, source: IoError },
    #[error(transparent)]
    Io(#[from] IoError),
    #[error("invalid OAuth callback query `{query}`: {source}")]
    InvalidQuery {
        query: String,
        #[source]
        source: serde_urlencoded::de::Error,
    },
    #[error("unexpected HTTP method: expected `GET`, got {method}")]
    UnexpectedMethod { method: Method },
    #[error("unexpected path: expected `{expected}`, got `{actual}`")]
    UnexpectedPath { expected: String, actual: String },
    #[error("server received shutdown signal")]
    Shutdown,
    #[error("timeout waiting for OAuth authorization callback")]
    Timeout,
}

impl ServerError {
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }

    pub fn is_invalid_query(&self) -> bool {
        matches!(self, Self::InvalidQuery { .. })
    }

    pub fn is_unexpected_method(&self) -> bool {
        matches!(self, Self::UnexpectedMethod { .. })
    }

    pub fn is_unexpected_path(&self) -> bool {
        matches!(self, Self::UnexpectedPath { .. })
    }

    pub fn is_shutdown(&self) -> bool {
        matches!(self, Self::Shutdown)
    }

    pub fn is_bind_failed(&self) -> bool {
        matches!(self, Self::BindFailed { .. })
    }

    pub fn is_io(&self) -> bool {
        matches!(self, Self::Io(_))
    }

    /// Returns the query string if this is an [`InvalidQuery`](ServerError::InvalidQuery) error.
    pub fn query(&self) -> Option<&str> {
        match self {
            Self::InvalidQuery { query, source: _ } => Some(query),
            _ => None,
        }
    }

    /// Returns the HTTP method if this is an [`UnexpectedMethod`](ServerError::UnexpectedMethod) error.
    pub fn method(&self) -> Option<&Method> {
        match self {
            Self::UnexpectedMethod { method } => Some(method),
            _ => None,
        }
    }

    /// Returns a tuple of `(expected, actual)` paths if this is an [`UnexpectedPath`](ServerError::UnexpectedPath) error.
    pub fn path(&self) -> Option<(&str, &str)> {
        match self {
            Self::UnexpectedPath { expected, actual } => Some((expected, actual)),
            _ => None,
        }
    }
}
