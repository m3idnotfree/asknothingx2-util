#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api")]
pub mod api;
#[cfg(any(feature = "oauth", feature = "oauth-server"))]
pub mod oauth;
#[cfg(feature = "serde")]
pub mod serde;
