//! Aurora GTK4 Renderer
//!
//! Renders Aurora design system in GTK4 applications.
//! Provides:
//! - CSS provider (design tokens → CSS)
//! - Widget library (Button, Card, Dialog, etc.)
//! - Animation integration
//! - Accessibility support

pub mod errors;
pub mod css;
pub mod theme;
pub mod widgets;
pub mod motion;
pub mod gnome;
pub mod icons;
pub mod storybook;
pub mod cli;

pub use errors::{GtkError, GtkResult};
pub use css::CssProvider;
pub use theme::Theme;
pub use widgets::{
    Button, ButtonStyle, ButtonState,
    Card, CardStyle,
    Input, InputType,
    AuroraDialog, DialogResponse,
    Checkbox,
    RadioButton,
    set_tooltip, remove_tooltip,
    List,
    Badge, BadgeStyle,
    Sidebar,
};

/// Aurora GTK4 renderer context
#[derive(Debug)]
pub struct AuroraGtk {
    css_provider: CssProvider,
    current_theme: Theme,
}

impl AuroraGtk {
    /// Create a new Aurora GTK context
    pub fn new(theme: Theme) -> GtkResult<Self> {
        let css_provider = CssProvider::new(theme)?;

        Ok(Self {
            css_provider,
            current_theme: theme,
        })
    }

    /// Get the current theme
    pub fn current_theme(&self) -> Theme {
        self.current_theme
    }

    /// Switch theme
    pub fn set_theme(&mut self, theme: Theme) -> GtkResult<()> {
        self.css_provider = CssProvider::new(theme)?;
        self.current_theme = theme;
        Ok(())
    }

    /// Get CSS provider
    pub fn css_provider(&self) -> &CssProvider {
        &self.css_provider
    }

    /// Get CSS string for all styles
    pub fn get_css(&self) -> String {
        self.css_provider.generate_css()
    }
}

impl Default for AuroraGtk {
    fn default() -> Self {
        Self::new(Theme::Light).expect("Default theme creation should not fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aurora_gtk_creation() {
        let aurora = AuroraGtk::new(Theme::Light);
        assert!(aurora.is_ok());
    }

    #[test]
    fn test_aurora_gtk_default() {
        let aurora = AuroraGtk::default();
        assert_eq!(aurora.current_theme(), Theme::Light);
    }

    #[test]
    fn test_aurora_gtk_theme_switch() {
        let mut aurora = AuroraGtk::new(Theme::Light).unwrap();
        aurora.set_theme(Theme::Dark).unwrap();
        assert_eq!(aurora.current_theme(), Theme::Dark);
    }

    #[test]
    fn test_aurora_gtk_css_generation() {
        let aurora = AuroraGtk::new(Theme::Light).unwrap();
        let css = aurora.get_css();
        assert!(!css.is_empty());
    }
}
