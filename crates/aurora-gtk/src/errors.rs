use thiserror::Error;

pub type GtkResult<T> = Result<T, GtkError>;

#[derive(Debug, Error)]
pub enum GtkError {
    #[error("GTK initialization failed: {0}")]
    InitializationError(String),

    #[error("CSS provider error: {0}")]
    CssError(String),

    #[error("Widget error: {0}")]
    WidgetError(String),

    #[error("Theme error: {0}")]
    ThemeError(String),

    #[error("Wayland error: {0}")]
    WaylandError(String),
}
