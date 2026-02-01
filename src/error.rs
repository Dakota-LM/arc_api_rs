use reqwest::StatusCode;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetaForgeError {
    /// Request failed at the transport layer (DNS, connect, TLS, timeout, etc.)
    #[error("http transport error: {0}")]
    Transport(#[source] reqwest::Error),

    /// Any other reqwest error (commonly JSON decode / body read).
    #[error("http client error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Server returned a non-success status code (4xx/5xx handled as hard error).
    #[error("unexpected http status: {0}")]
    HttpStatus(StatusCode),

    /// We got 429s (or were otherwise throttled) too many times.
    #[error("rate limited; wait {wait:?} before retrying")]
    RateLimited { wait: Duration },

    /// Something about the base URL / path composition was invalid.
    #[error("invalid base url: {0}")]
    InvalidBaseUrl(String),
}

// Optional convenience helpers if you want them later.
impl MetaForgeError {
    /// True if this looks like a temporary issue that might succeed later.
    pub fn is_transient(&self) -> bool {
        match self {
            MetaForgeError::Transport(_) => true,
            MetaForgeError::HttpStatus(code) => code.is_server_error(),
            MetaForgeError::RateLimited { .. } => true,
            _ => false,
        }
    }
}
