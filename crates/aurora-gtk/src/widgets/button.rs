use gtk::{prelude::*, Button as GtkButton, Orientation};
use crate::motion::GtkAnimator;
use crate::css::CssProvider;

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
pub struct Button {
    inner: GtkButton,
    style: ButtonStyle,
    state: ButtonState,
    animator: GtkAnimator,
    css_provider: CssProvider,
}

impl Button {
    /// Create a new button with label text
    pub fn new(label: &str) -> Self {
        let button = GtkButton::builder()
            .label(label)
            .build();

        let animator = GtkAnimator::new();
        let css_provider = CssProvider::new();

        Self {
            inner: button,
            style: ButtonStyle::default(),
            state: ButtonState::Default,
            animator,
            css_provider,
        }
    }

    /// Set button style
    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self.apply_style();
        self
    }

    /// Set button label
    pub fn with_label(self, label: &str) -> Self {
        self.inner.set_label(label);
        self
    }

    /// Enable/disable the button
    pub fn set_sensitive(self, sensitive: bool) -> Self {
        self.inner.set_sensitive(sensitive);
        if !sensitive {
            self.state = ButtonState::Disabled;
        }
        self
    }

    /// Add CSS class for styling
    pub fn add_css_class(self, class: &str) -> Self {
        self.inner.add_css_class(class);
        self
    }

    /// Connect to clicked signal
    pub fn connect_clicked<F: Fn() + 'static>(self, callback: F) -> Self {
        self.inner.connect_clicked(move |_| callback());
        self
    }

    /// Get reference to inner GTK button
    pub fn inner(&self) -> &GtkButton {
        &self.inner
    }

    /// Get mutable reference to inner GTK button
    pub fn inner_mut(&mut self) -> &mut GtkButton {
        &mut self.inner
    }

    /// Apply style based on current style variant
    fn apply_style(&self) {
        // Clear previous classes
        self.inner.remove_css_class("aurora-button-filled");
        self.inner.remove_css_class("aurora-button-tinted");
        self.inner.remove_css_class("aurora-button-outlined");
        self.inner.remove_css_class("aurora-button-ghost");

        // Add appropriate class
        let class = match self.style {
            ButtonStyle::Filled => "aurora-button-filled",
            ButtonStyle::Tinted => "aurora-button-tinted",
            ButtonStyle::Outlined => "aurora-button-outlined",
            ButtonStyle::Ghost => "aurora-button-ghost",
        };

        self.inner.add_css_class(class);
    }

    /// Animate button press
    pub fn animate_press(&self) {
        self.animator.animate_scale(self.inner.clone(), 0.98, 1.0, 100);
    }

    /// Animate hover
    pub fn animate_hover_in(&self) {
        self.animator.animate_opacity(self.inner.clone(), 0.8, 1.0, 150);
    }

    /// Animate hover out
    pub fn animate_hover_out(&self) {
        self.animator.animate_opacity(self.inner.clone(), 1.0, 0.8, 150);
    }
}

impl AsRef<GtkButton> for Button {
    fn as_ref(&self) -> &GtkButton {
        &self.inner
    }
}

impl From<Button> for GtkButton {
    fn from(button: Button) -> Self {
        button.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_new() {
        let button = Button::new("Test");
        assert_eq!(button.inner.label(), Some("Test".to_string()));
    }

    #[test]
    fn test_button_style_filled() {
        let button = Button::new("Test").with_style(ButtonStyle::Filled);
        assert_eq!(button.style, ButtonStyle::Filled);
    }

    #[test]
    fn test_button_style_tinted() {
        let button = Button::new("Test").with_style(ButtonStyle::Tinted);
        assert_eq!(button.style, ButtonStyle::Tinted);
    }

    #[test]
    fn test_button_style_outlined() {
        let button = Button::new("Test").with_style(ButtonStyle::Outlined);
        assert_eq!(button.style, ButtonStyle::Outlined);
    }

    #[test]
    fn test_button_style_ghost() {
        let button = Button::new("Test").with_style(ButtonStyle::Ghost);
        assert_eq!(button.style, ButtonStyle::Ghost);
    }

    #[test]
    fn test_button_default_style() {
        let button = Button::new("Test");
        assert_eq!(button.style, ButtonStyle::Filled);
    }

    #[test]
    fn test_button_label_update() {
        let button = Button::new("Initial").with_label("Updated");
        assert_eq!(button.inner.label(), Some("Updated".to_string()));
    }

    #[test]
    fn test_button_disabled() {
        let button = Button::new("Test").set_sensitive(false);
        assert!(!button.inner.is_sensitive());
        assert_eq!(button.state, ButtonState::Disabled);
    }

    #[test]
    fn test_button_enabled() {
        let button = Button::new("Test").set_sensitive(true);
        assert!(button.inner.is_sensitive());
    }

    #[test]
    fn test_button_css_class() {
        let button = Button::new("Test").add_css_class("custom-class");
        assert!(button.inner.has_css_class("custom-class"));
    }

    #[test]
    fn test_button_state_default() {
        let button = Button::new("Test");
        assert_eq!(button.state, ButtonState::Default);
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
