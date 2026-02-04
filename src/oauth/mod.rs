#[cfg(feature = "oauth-server")]
pub mod oneshot;

#[cfg(feature = "oauth")]
pub mod signed_token;

#[cfg(feature = "oauth")]
mod new_types;

#[cfg(feature = "oauth")]
pub use new_types::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RefreshToken, RevocationUrl, Scope, TokenUrl, ValidateUrl,
};
