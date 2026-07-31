use thiserror::Error;

pub type TokenResult<T> = Result<T, TokenError>;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Contrast ratio validation failed: {0}")]
    ContrastRatioError(String),

    #[error("Invalid token value: {0}")]
    InvalidValue(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("Token not found: {0}")]
    TokenNotFound(String),

    #[error("Accessibility violation: {0}")]
    AccessibilityViolation(String),
}
