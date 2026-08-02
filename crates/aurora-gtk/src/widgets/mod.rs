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
pub use tabs::{Tab, TabOrientation, TabPanel, TabStyle, Tabs};
pub use tooltip::{remove_tooltip, set_tooltip};
