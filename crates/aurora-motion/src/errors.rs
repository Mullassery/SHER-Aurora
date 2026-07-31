use thiserror::Error;

pub type MotionResult<T> = Result<T, MotionError>;

#[derive(Debug, Error)]
pub enum MotionError {
    #[error("Invalid animation duration: {0}")]
    InvalidDuration(String),

    #[error("Invalid spring parameter: {0}")]
    InvalidSpringParameter(String),

    #[error("Invalid easing curve: {0}")]
    InvalidEasingCurve(String),

    #[error("Animation not found: {0}")]
    AnimationNotFound(String),

    #[error("Invalid animation state: {0}")]
    InvalidState(String),
}
