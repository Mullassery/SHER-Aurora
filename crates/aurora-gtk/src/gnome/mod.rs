//! GNOME integration layer

pub mod dconf;
pub mod observer;
pub mod notifications;
pub mod settings_panel;

pub use dconf::DConfSchema;
pub use observer::ThemeObserver;
pub use notifications::{AuroraNotification, NotificationManager, NotificationUrgency};
pub use settings_panel::{SettingsPanel, SettingsSection, Setting, SettingType};
