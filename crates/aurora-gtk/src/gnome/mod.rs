//! GNOME integration layer

pub mod dconf;
pub mod notifications;
pub mod observer;
pub mod settings_panel;

pub use dconf::DConfSchema;
pub use notifications::{AuroraNotification, NotificationManager, NotificationUrgency};
pub use observer::ThemeObserver;
pub use settings_panel::{Setting, SettingType, SettingsPanel, SettingsSection};
