/// Aurora Badge styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStyle {
    /// Default badge style
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

impl Default for BadgeStyle {
    fn default() -> Self {
        Self::Default
    }
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

    #[test]
    fn test_badge_chaining() {
        let _badge = Badge::new("Status")
            .with_style(BadgeStyle::Success)
            .with_text("Active")
            .add_css_class("test");
    }
}
