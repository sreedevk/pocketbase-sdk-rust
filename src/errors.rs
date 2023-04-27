use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication Failed")]
    AuthenticationFailed,
    #[error("Auth Response Parse Error")]
    AuthResponseParseFailed,
}

