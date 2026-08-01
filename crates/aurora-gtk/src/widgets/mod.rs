//! Aurora GTK4 Component Library
//!
//! Production-ready widgets for building beautiful GNOME applications.
//!
//! # Components
//!
//! - **Button** — Filled, tinted, outlined, ghost variants
//! - **Card** — Elevated, outlined container
//! - **Input** — Text, password, search fields
//! - **Dialog** — Modal and non-blocking dialogs
//! - **Checkbox** — Selectable checkbox
//! - **Radio** — Radio button selection
//! - **Tooltip** — Informational tooltips
//! - **List** — Scrollable list container
//! - **Badge** — Status indicators
//! - **Sidebar** — Navigation sidebar
//!
//! # Example
//!
//! ```rust,no_run
//! use aurora_gtk::widgets::{Button, ButtonStyle};
//!
//! let button = Button::new("Click me")
//!     .with_style(ButtonStyle::Filled);
//! ```

pub mod button;

pub use button::{Button, ButtonStyle, ButtonState};

// Future components
// pub mod card;
// pub mod input;
// pub mod dialog;
// pub mod checkbox;
// pub mod radio;
// pub mod tooltip;
// pub mod list;
// pub mod badge;
// pub mod sidebar;
