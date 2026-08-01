//! GNOME notification styling integration

use aurora_color::ThemeName;

/// Notification urgency level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    High,
}

impl NotificationUrgency {
    /// Get urgency as D-Bus integer
    pub fn as_dbus_int(&self) -> i32 {
        match self {
            NotificationUrgency::Low => 0,
            NotificationUrgency::Normal => 1,
            NotificationUrgency::High => 2,
        }
    }
}

/// Aurora-styled GNOME notification
pub struct AuroraNotification {
    summary: String,
    body: String,
    urgency: NotificationUrgency,
    timeout: i32, // milliseconds, -1 = default
    theme: ThemeName,
}

impl AuroraNotification {
    /// Create new notification
    pub fn new(summary: &str) -> Self {
        Self {
            summary: summary.to_string(),
            body: String::new(),
            urgency: NotificationUrgency::Normal,
            timeout: -1,
            theme: ThemeName::Light,
        }
    }

    /// Set notification body
    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    /// Set urgency level
    pub fn with_urgency(mut self, urgency: NotificationUrgency) -> Self {
        self.urgency = urgency;
        self
    }

    /// Set timeout in milliseconds
    pub fn with_timeout(mut self, timeout: i32) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set theme for styling
    pub fn with_theme(mut self, theme: ThemeName) -> Self {
        self.theme = theme;
        self
    }

    /// Get summary
    pub fn summary(&self) -> &str {
        &self.summary
    }

    /// Get body
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Get urgency
    pub fn urgency(&self) -> NotificationUrgency {
        self.urgency
    }

    /// Get timeout
    pub fn timeout(&self) -> i32 {
        self.timeout
    }

    /// Get theme
    pub fn theme(&self) -> ThemeName {
        self.theme
    }

    /// Generate CSS for notification styling
    pub fn to_css(&self) -> String {
        let bg_color = match self.theme {
            ThemeName::Light => "#FEFEFE",
            ThemeName::Dark | ThemeName::OLED => "#1E1E1E",
            ThemeName::HDR => "#1E1E1E",
        };

        let text_color = match self.theme {
            ThemeName::Light => "#1A1A1A",
            ThemeName::Dark | ThemeName::OLED => "#F5F5F5",
            ThemeName::HDR => "#F5F5F5",
        };

        let border_color = match self.urgency {
            NotificationUrgency::Low => "#004400",
            NotificationUrgency::Normal => "#0066CC",
            NotificationUrgency::High => "#990000",
        };

        format!(
            r#".aurora-notification {{
  background-color: {};
  color: {};
  border-left: 4px solid {};
  padding: 12px 16px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}}"#,
            bg_color, text_color, border_color
        )
    }
}

impl Default for AuroraNotification {
    fn default() -> Self {
        Self::new("Notification")
    }
}

/// Notification manager
pub struct NotificationManager {
    theme: ThemeName,
    notifications: Vec<AuroraNotification>,
}

impl NotificationManager {
    /// Create new notification manager
    pub fn new(theme: ThemeName) -> Self {
        Self {
            theme,
            notifications: Vec::new(),
        }
    }

    /// Create and queue notification
    pub fn notify(&mut self, summary: &str) {
        let notification = AuroraNotification::new(summary).with_theme(self.theme);
        self.notifications.push(notification);
    }

    /// Get pending notifications
    pub fn pending(&self) -> &[AuroraNotification] {
        &self.notifications
    }

    /// Clear notifications
    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    /// Update theme
    pub fn set_theme(&mut self, theme: ThemeName) {
        self.theme = theme;
    }

    /// Get notification count
    pub fn count(&self) -> usize {
        self.notifications.len()
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new(ThemeName::Light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notif = AuroraNotification::new("Test");
        assert_eq!(notif.summary(), "Test");
        assert_eq!(notif.urgency(), NotificationUrgency::Normal);
    }

    #[test]
    fn test_notification_builder() {
        let notif = AuroraNotification::new("Title")
            .with_body("Body text")
            .with_urgency(NotificationUrgency::High)
            .with_timeout(5000);

        assert_eq!(notif.summary(), "Title");
        assert_eq!(notif.body(), "Body text");
        assert_eq!(notif.urgency(), NotificationUrgency::High);
        assert_eq!(notif.timeout(), 5000);
    }

    #[test]
    fn test_urgency_dbus_int() {
        assert_eq!(NotificationUrgency::Low.as_dbus_int(), 0);
        assert_eq!(NotificationUrgency::Normal.as_dbus_int(), 1);
        assert_eq!(NotificationUrgency::High.as_dbus_int(), 2);
    }

    #[test]
    fn test_notification_css_light() {
        let notif = AuroraNotification::new("Test").with_theme(ThemeName::Light);
        let css = notif.to_css();
        assert!(css.contains("#FEFEFE"));
        assert!(css.contains("#1A1A1A"));
    }

    #[test]
    fn test_notification_css_dark() {
        let notif = AuroraNotification::new("Test").with_theme(ThemeName::Dark);
        let css = notif.to_css();
        assert!(css.contains("#1E1E1E"));
        assert!(css.contains("#F5F5F5"));
    }

    #[test]
    fn test_notification_manager_creation() {
        let manager = NotificationManager::new(ThemeName::Light);
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_notification_manager_notify() {
        let mut manager = NotificationManager::new(ThemeName::Light);
        manager.notify("Test notification");
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_notification_manager_clear() {
        let mut manager = NotificationManager::new(ThemeName::Light);
        manager.notify("Test 1");
        manager.notify("Test 2");
        assert_eq!(manager.count(), 2);

        manager.clear();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_notification_manager_theme_change() {
        let mut manager = NotificationManager::new(ThemeName::Light);
        manager.set_theme(ThemeName::Dark);
        manager.notify("Test");
        assert_eq!(manager.pending()[0].theme(), ThemeName::Dark);
    }

    #[test]
    fn test_default() {
        let notif = AuroraNotification::default();
        assert_eq!(notif.summary(), "Notification");
    }
}
