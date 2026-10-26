use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

// use http::header::ToStrError;
use http::header::InvalidHeaderValue;

pub struct Error {
    inner: Box<Inner>,
}

type BoxError = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    source: Option<BoxError>,
}

enum Kind {
    AuthScheme { scheme: String },
    // MimeType(MimeType),
}

// enum MimeType {
//     ToStr,
//     TooLong,
//     TooLongWithParams,
//     Malformed { input: String },
//     Unsupported { input: String },
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    AuthScheme,
    // MimeType,
}

impl ErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorKind::AuthScheme => "auth_scheme",
            // ErrorKind::MimeType => "mime_type",
        }
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self.inner.kind {
            Kind::AuthScheme { .. } => ErrorKind::AuthScheme,
            // Kind::MimeType(_) => ErrorKind::MimeType,
        }
    }

    fn new(kind: Kind, source: Option<BoxError>) -> Self {
        Self {
            inner: Box::new(Inner { kind, source }),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut builder = f.debug_struct("asknothingx2_util::api::Error");

        builder.field("kind", &self.kind());
        builder.field("message", &self.to_string());

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.inner.kind {
            Kind::AuthScheme { scheme } => {
                write!(
                    f,
                    "invalid authorization header value for scheme {scheme:?}"
                )
            } // Kind::MimeType(reason) => match reason {
              //     MimeType::ToStr => f.write_str("failed to convert header value to a string"),
              //     MimeType::TooLong => f.write_str("MIME type too long"),
              //     MimeType::TooLongWithParams => f.write_str("MIME type with parameters too long"),
              //     MimeType::Malformed { input } => {
              //         write!(f, "malformed MIME type {input:?}")
              //     }
              //     MimeType::Unsupported { input } => {
              //         write!(f, "unsupported MIME type {input:?}")
              //     }
              // },
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

pub(crate) fn invalid_auth_scheme(scheme: impl Into<String>, source: InvalidHeaderValue) -> Error {
    Error::new(
        Kind::AuthScheme {
            scheme: scheme.into(),
        },
        Some(source.into()),
    )
}

// pub(crate) fn mime_type_to_str(source: ToStrError) -> Error {
//     Error::new(Kind::MimeType(MimeType::ToStr), Some(source.into()))
// }
//
// pub(crate) fn mime_type_too_long() -> Error {
//     Error::new(Kind::MimeType(MimeType::TooLong), None)
// }
//
// pub(crate) fn mime_type_with_params_too_long() -> Error {
//     Error::new(Kind::MimeType(MimeType::TooLongWithParams), None)
// }
//
// pub(crate) fn mime_type_malformed(input: impl Into<String>) -> Error {
//     Error::new(
//         Kind::MimeType(MimeType::Malformed {
//             input: input.into(),
//         }),
//         None,
//     )
// }
//
// pub(crate) fn mime_type_unsupported(input: impl Into<String>) -> Error {
//     Error::new(
//         Kind::MimeType(MimeType::Unsupported {
//             input: input.into(),
//         }),
//         None,
//     )
// }

#[cfg(test)]
mod tests {
    use crate::api::AuthScheme;

    use super::*;

    #[test]
    fn send_sync_static() {
        fn assert<T: Send + Sync + 'static>() {}
        assert::<Error>();
    }

    #[test]
    fn invalid_header_error_does_not_leak_credentials() {
        let err = AuthScheme::custom("OAuth", "secret\n")
            .to_header_value()
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            "invalid authorization header value for scheme \"OAuth\""
        );
        assert!(!format!("{err:?}").contains("secret\n"));
    }
}
