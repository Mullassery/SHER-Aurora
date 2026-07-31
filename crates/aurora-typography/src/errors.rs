use thiserror::Error;

pub type TypographyResult<T> = Result<T, TypographyError>;

#[derive(Debug, Error)]
pub enum TypographyError {
    #[error("Font not found: {0}")]
    FontNotFound(String),

    #[error("Invalid font size: {0}")]
    InvalidFontSize(String),

    #[error("Invalid line height: {0}")]
    InvalidLineHeight(String),

    #[error("Invalid letter spacing: {0}")]
    InvalidLetterSpacing(String),

    #[error("Type scale validation failed: {0}")]
    ValidationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Font loading error: {0}")]
    FontLoadError(String),
}
