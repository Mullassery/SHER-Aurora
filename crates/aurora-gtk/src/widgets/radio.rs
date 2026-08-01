/// Aurora Radio Button component
///
/// A radio button for single-selection within a group.
#[derive(Debug, Clone)]
pub struct RadioButton {
    label: String,
    selected: bool,
    sensitive: bool,
    css_classes: Vec<String>,
}

impl RadioButton {
    /// Create a new radio button
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            selected: false,
            sensitive: true,
            css_classes: vec!["aurora-radio".to_string()],
        }
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Get selected state
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Set label
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    /// Enable/disable
    pub fn set_sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = sensitive;
        self
    }

    /// Add CSS class
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get label text
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }
}

impl Default for RadioButton {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_new() {
        let radio = RadioButton::new("Option");
        assert_eq!(radio.label(), "Option");
    }

    #[test]
    fn test_radio_default_unselected() {
        let radio = RadioButton::new("Option");
        assert!(!radio.is_selected());
    }

    #[test]
    fn test_radio_selected() {
        let radio = RadioButton::new("Option").selected(true);
        assert!(radio.is_selected());
    }

    #[test]
    fn test_radio_label_update() {
        let radio = RadioButton::new("Initial")
            .with_label("Updated");
        assert_eq!(radio.label(), "Updated");
    }

    #[test]
    fn test_radio_css_class() {
        let radio = RadioButton::new("Option").add_css_class("custom");
        assert!(radio.css_classes().contains(&"custom".to_string()));
    }

    #[test]
    fn test_radio_chaining() {
        let _radio = RadioButton::new("Option")
            .selected(false)
            .set_sensitive(true)
            .add_css_class("test");
    }
}
