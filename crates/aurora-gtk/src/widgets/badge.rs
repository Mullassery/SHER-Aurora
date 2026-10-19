/// Aurora Badge styles
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStyle {
    /// Default badge style
    #[default]
    Default,
    /// Success state (green)
    Success,
    /// Warning state (yellow)
    Warning,
    /// Error state (red)
    Error,
    /// Info state (blue)
    Info,
}

/// Aurora Badge component
///
/// A small status indicator with multiple style variants.
#[derive(Debug, Clone)]
pub struct Badge {
    text: String,
    style: BadgeStyle,
    css_classes: Vec<String>,
}

impl Badge {
    /// Create a new badge
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            style: BadgeStyle::default(),
            css_classes: vec!["aurora-badge".to_string()],
        }
    }

    /// Set badge style
    pub fn with_style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }

    /// Update badge text
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    /// Add CSS class
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get badge text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get badge style
    pub fn style(&self) -> BadgeStyle {
        self.style
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }

    /// Build a real `gtk4::Label` styled as an Aurora badge.
    ///
    /// GTK must already be initialized. The badge is a compact label with
    /// `aurora-badge` plus a per-style class so the generated stylesheet can
    /// select the correct foreground/background.
    pub fn build(&self) -> gtk4::Label {
        use gtk4::prelude::*;

        let label = gtk4::Label::new(Some(&self.text));
        label.set_xalign(0.5);
        for class in &self.css_classes {
            label.add_css_class(class);
        }
        match self.style {
            BadgeStyle::Default => label.add_css_class("aurora-badge-default"),
            BadgeStyle::Success => label.add_css_class("aurora-badge-success"),
            BadgeStyle::Warning => label.add_css_class("aurora-badge-warning"),
            BadgeStyle::Error => label.add_css_class("aurora-badge-error"),
            BadgeStyle::Info => label.add_css_class("aurora-badge-info"),
        }
        label
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_new() {
        let badge = Badge::new("Test");
        assert_eq!(badge.text(), "Test");
    }

    #[test]
    fn test_badge_default_style() {
        let badge = Badge::new("Test");
        assert_eq!(badge.style(), BadgeStyle::Default);
    }

    #[test]
    fn test_badge_success_style() {
        let badge = Badge::new("Test").with_style(BadgeStyle::Success);
        assert_eq!(badge.style(), BadgeStyle::Success);
    }

    #[test]
    fn test_badge_text_update() {
        let badge = Badge::new("Initial").with_text("Updated");
        assert_eq!(badge.text(), "Updated");
    }

    #[test]
    fn test_badge_css_class() {
        let badge = Badge::new("Test").add_css_class("custom");
        assert!(badge.css_classes().contains(&"custom".to_string()));
    }

    #[cfg(not(target_os = "macos"))]
    #[gtk4::test]
    fn test_badge_builds_real_gtk_label() {
        use gtk4::prelude::*;

        let _ = gtk4::init();
        let label = Badge::new("Active").with_style(BadgeStyle::Success).build();
        assert_eq!(label.label().to_string(), "Active");
        assert!(label.has_css_class("aurora-badge-success"));
    }

    #[test]
    fn test_badge_chaining() {
        let _badge = Badge::new("Status")
            .with_style(BadgeStyle::Success)
            .with_text("Active")
            .add_css_class("test");
    }
}
