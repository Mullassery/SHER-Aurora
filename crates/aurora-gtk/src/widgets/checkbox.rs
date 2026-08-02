/// Aurora Checkbox component
///
/// A selectable checkbox with support for checked, unchecked, and indeterminate states.
#[derive(Debug, Clone)]
pub struct Checkbox {
    label: String,
    checked: bool,
    inconsistent: bool,
    sensitive: bool,
    css_classes: Vec<String>,
}

impl Checkbox {
    /// Create a new checkbox
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            checked: false,
            inconsistent: false,
            sensitive: true,
            css_classes: vec!["aurora-checkbox".to_string()],
        }
    }

    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Get checked state
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    /// Set inconsistent (indeterminate) state
    pub fn inconsistent(mut self, inconsistent: bool) -> Self {
        self.inconsistent = inconsistent;
        self
    }

    /// Check if inconsistent state
    pub fn is_inconsistent(&self) -> bool {
        self.inconsistent
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

impl Default for Checkbox {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_new() {
        let checkbox = Checkbox::new("Test");
        assert_eq!(checkbox.label(), "Test");
    }

    #[test]
    fn test_checkbox_default_unchecked() {
        let checkbox = Checkbox::new("Test");
        assert!(!checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_checked() {
        let checkbox = Checkbox::new("Test").checked(true);
        assert!(checkbox.is_checked());
    }

    #[test]
    fn test_checkbox_inconsistent() {
        let checkbox = Checkbox::new("Test").inconsistent(true);
        assert!(checkbox.is_inconsistent());
    }

    #[test]
    fn test_checkbox_label_update() {
        let checkbox = Checkbox::new("Initial").with_label("Updated");
        assert_eq!(checkbox.label(), "Updated");
    }

    #[test]
    fn test_checkbox_css_class() {
        let checkbox = Checkbox::new("Test").add_css_class("custom");
        assert!(checkbox.css_classes().contains(&"custom".to_string()));
    }

    #[test]
    fn test_checkbox_chaining() {
        let _checkbox = Checkbox::new("Accept")
            .checked(false)
            .inconsistent(false)
            .set_sensitive(true)
            .add_css_class("test");
    }
}
