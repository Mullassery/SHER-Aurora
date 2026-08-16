//! Aurora GTK4 Component Library
//!
//! Production-ready widgets for building beautiful GNOME applications.
//!
//! # Components (v1.0)
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
//! - **Switch** — Binary on/off toggle
//!
//! # Real GTK4 rendering
//!
//! Button, Input, Checkbox, Card, and Switch each expose a `build()` method
//! that constructs a real `gtk4` widget object (`gtk4::Button`,
//! `gtk4::Entry`, `gtk4::CheckButton`, `gtk4::Box`, `gtk4::Switch`
//! respectively), backed by this crate's real `gtk4` dependency — not a
//! mock or a lookalike struct. Callers must initialize GTK first (e.g.
//! `gtk4::init()` or a running `gtk4::Application`). See
//! `examples/gtk4_harness.rs` for a runnable end-to-end demonstration,
//! including installing Aurora's token-derived CSS onto a real
//! `gtk4::gdk::Display`.
//!
//! The remaining widgets below are still logic-only descriptors (styling
//! and state modeling backed by the Aurora token/typography/motion system)
//! without a `build()` method yet; expanding real-GTK4 coverage to them is
//! tracked as follow-up work rather than claimed as done.
//!
//! # Components (v1.1)
//!
//! - **DataTable** — Sortable, selectable data display with pagination
//! - **Tabs** — Multi-view navigation with multiple styles and animations
//! - **Select** — Dropdown with search, single/multi-select, custom rendering
//! - **Menu** — Context and navigation menus with keyboard shortcuts
//! - **Breadcrumb** — File path and hierarchical navigation
//! - **IconDock** — Animated icon navigation bar with spring physics
//!
//! # Example
//!
//! ```rust,no_run
//! use aurora_gtk::widgets::{Button, ButtonStyle};
//!
//! let button = Button::new("Click me")
//!     .with_style(ButtonStyle::Filled);
//! ```

pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod datatable;
pub mod dialog;
pub mod icon_dock;
pub mod input;
pub mod list;
pub mod menu;
pub mod radio;
pub mod select;
pub mod sidebar;
pub mod switch;
pub mod tabs;
pub mod tooltip;

pub use badge::{Badge, BadgeStyle};
pub use breadcrumb::{Breadcrumb, OverflowBehavior, Segment};
pub use button::{Button, ButtonState, ButtonStyle};
pub use card::{Card, CardStyle};
pub use checkbox::Checkbox;
pub use datatable::{Column, DataTable, Row, SelectionMode, SortDirection};
pub use dialog::{AuroraDialog, DialogResponse};
pub use icon_dock::{DockAnimation, DockItem, DockOrientation, DockPosition, IconDock};
pub use input::{Input, InputType};
pub use list::List;
pub use menu::{Menu, MenuItem, MenuItemState, MenuItemType, Shortcut};
pub use radio::RadioButton;
pub use select::{OptionState, Select, SelectMode, SelectOption};
pub use sidebar::Sidebar;
pub use switch::Switch;
pub use tabs::{Tab, TabOrientation, TabPanel, TabStyle, Tabs};
pub use tooltip::{remove_tooltip, set_tooltip};
