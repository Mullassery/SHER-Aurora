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

pub mod button;
pub mod card;
pub mod input;
pub mod dialog;
pub mod checkbox;
pub mod radio;
pub mod tooltip;
pub mod list;
pub mod badge;
pub mod sidebar;
pub mod datatable;
pub mod tabs;
pub mod select;
pub mod menu;
pub mod breadcrumb;
pub mod icon_dock;

pub use button::{Button, ButtonStyle, ButtonState};
pub use card::{Card, CardStyle};
pub use input::{Input, InputType};
pub use dialog::{AuroraDialog, DialogResponse};
pub use checkbox::Checkbox;
pub use radio::RadioButton;
pub use tooltip::{set_tooltip, remove_tooltip};
pub use list::List;
pub use badge::{Badge, BadgeStyle};
pub use sidebar::Sidebar;
pub use datatable::{DataTable, Column, Row, SortDirection, SelectionMode};
pub use tabs::{Tabs, Tab, TabPanel, TabStyle, TabOrientation};
pub use select::{Select, SelectOption, SelectMode, OptionState};
pub use menu::{Menu, MenuItem, MenuItemType, MenuItemState, Shortcut};
pub use breadcrumb::{Breadcrumb, Segment, OverflowBehavior};
pub use icon_dock::{IconDock, DockItem, DockOrientation, DockPosition, DockAnimation};
