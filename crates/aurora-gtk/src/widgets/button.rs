/// Aurora Button styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Solid fill with primary color
    Filled,
    /// Tinted background with primary color
    Tinted,
    /// Border only, transparent background
    Outlined,
    /// Transparent with text only
    Ghost,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Filled
    }
}

/// Aurora Button states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Default,
    Hover,
    Active,
    Disabled,
    Loading,
}

/// Aurora Button component
///
/// A versatile button widget with four style variants:
/// - Filled: Solid primary color (default action)
/// - Tinted: Subtle background (secondary action)
/// - Outlined: Border only (tertiary action)
/// - Ghost: Text only (minimal action)
///
/// # Example
///
/// ```rust
/// use aurora_gtk::widgets::Button;
/// use aurora_gtk::widgets::ButtonStyle;
///
/// let button = Button::new("Click me")
///     .with_style(ButtonStyle::Filled);
/// ```
#[derive(Debug, Clone)]
pub struct Button {
    label: String,
    style: ButtonStyle,
    state: ButtonState,
    sensitive: bool,
    css_classes: Vec<String>,
}

impl Button {
    /// Create a new button with label text
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            style: ButtonStyle::default(),
            state: ButtonState::Default,
            sensitive: true,
            css_classes: vec!["aurora-button".to_string()],
        }
    }

    /// Set button style
    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Set button label
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    /// Enable/disable the button
    pub fn set_sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = sensitive;
        if !sensitive {
            self.state = ButtonState::Disabled;
        }
        self
    }

    /// Add CSS class for styling
    pub fn add_css_class(mut self, class: &str) -> Self {
        self.css_classes.push(class.to_string());
        self
    }

    /// Get button label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get button style
    pub fn style(&self) -> ButtonStyle {
        self.style
    }

    /// Get button state
    pub fn state(&self) -> ButtonState {
        self.state
    }

    /// Check if button is sensitive
    pub fn is_sensitive(&self) -> bool {
        self.sensitive
    }

    /// Get CSS classes
    pub fn css_classes(&self) -> &[String] {
        &self.css_classes
    }

    /// Set state
    pub fn set_state(mut self, state: ButtonState) -> Self {
        self.state = state;
        self
    }

    /// Animate button press
    pub fn animate_press(&self) {
        // Animation would be handled by GTK4 renderer
    }

    /// Animate hover
    pub fn animate_hover_in(&self) {
        // Animation would be handled by GTK4 renderer
    }

    /// Animate hover out
    pub fn animate_hover_out(&self) {
        // Animation would be handled by GTK4 renderer
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_new() {
        let button = Button::new("Test");
        assert_eq!(button.label(), "Test");
    }

    #[test]
    fn test_button_style_filled() {
        let button = Button::new("Test").with_style(ButtonStyle::Filled);
        assert_eq!(button.style(), ButtonStyle::Filled);
    }

    #[test]
    fn test_button_style_tinted() {
        let button = Button::new("Test").with_style(ButtonStyle::Tinted);
        assert_eq!(button.style(), ButtonStyle::Tinted);
    }

    #[test]
    fn test_button_style_outlined() {
        let button = Button::new("Test").with_style(ButtonStyle::Outlined);
        assert_eq!(button.style(), ButtonStyle::Outlined);
    }

    #[test]
    fn test_button_style_ghost() {
        let button = Button::new("Test").with_style(ButtonStyle::Ghost);
        assert_eq!(button.style(), ButtonStyle::Ghost);
    }

    #[test]
    fn test_button_default_style() {
        let button = Button::new("Test");
        assert_eq!(button.style(), ButtonStyle::Filled);
    }

    #[test]
    fn test_button_label_update() {
        let button = Button::new("Initial").with_label("Updated");
        assert_eq!(button.label(), "Updated");
    }

    #[test]
    fn test_button_disabled() {
        let button = Button::new("Test").set_sensitive(false);
        assert!(!button.is_sensitive());
        assert_eq!(button.state(), ButtonState::Disabled);
    }

    #[test]
    fn test_button_enabled() {
        let button = Button::new("Test").set_sensitive(true);
        assert!(button.is_sensitive());
    }

    #[test]
    fn test_button_css_class() {
        let button = Button::new("Test").add_css_class("custom-class");
        assert!(button.css_classes().contains(&"custom-class".to_string()));
    }

    #[test]
    fn test_button_state_default() {
        let button = Button::new("Test");
        assert_eq!(button.state(), ButtonState::Default);
    }

    #[test]
    fn test_button_chaining() {
        let _button = Button::new("Test")
            .with_style(ButtonStyle::Outlined)
            .with_label("Updated")
            .add_css_class("test-class")
            .set_sensitive(true);
        // If this compiles, chaining works
    }
}
