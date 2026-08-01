//! Aurora Color System
//!
//! A comprehensive semantic color system with support for Light, Dark, OLED, and HDR themes.
//! All colors are defined as tokens, not raw hex values, ensuring consistency across applications.

pub mod color;
pub mod theme;
pub mod contrast;

pub use color::Color;
pub use theme::{Theme, ThemeName, ColorSystem};
pub use contrast::validate_contrast;

/// Aurora color system context
#[derive(Debug, Clone)]
pub struct AuroraColorSystem {
    theme: ThemeName,
    colors: ColorSystem,
    contrast_validated: bool,
}

impl AuroraColorSystem {
    /// Create a new color system with a theme
    pub fn new(theme: ThemeName) -> Self {
        let colors = ColorSystem::from_theme(theme);
        let contrast_validated = validate_contrast(&colors);

        Self {
            theme,
            colors,
            contrast_validated,
        }
    }

    /// Get current theme
    pub fn theme(&self) -> ThemeName {
        self.theme
    }

    /// Switch to a different theme
    pub fn set_theme(&mut self, theme: ThemeName) {
        self.theme = theme;
        self.colors = ColorSystem::from_theme(theme);
        self.contrast_validated = validate_contrast(&self.colors);
    }

    /// Get color system
    pub fn colors(&self) -> &ColorSystem {
        &self.colors
    }

    /// Check if contrast is validated
    pub fn is_contrast_validated(&self) -> bool {
        self.contrast_validated
    }

    /// Generate CSS custom properties
    pub fn to_css(&self) -> String {
        self.colors.to_css()
    }
}

impl Default for AuroraColorSystem {
    fn default() -> Self {
        Self::new(ThemeName::Light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_system_creation() {
        let system = AuroraColorSystem::new(ThemeName::Light);
        assert_eq!(system.theme(), ThemeName::Light);
    }

    #[test]
    fn test_color_system_default() {
        let system = AuroraColorSystem::default();
        assert_eq!(system.theme(), ThemeName::Light);
    }

    #[test]
    fn test_theme_switching() {
        let mut system = AuroraColorSystem::new(ThemeName::Light);
        system.set_theme(ThemeName::Dark);
        assert_eq!(system.theme(), ThemeName::Dark);
    }

    #[test]
    fn test_css_generation() {
        let system = AuroraColorSystem::new(ThemeName::Light);
        let css = system.to_css();
        assert!(!css.is_empty());
    }

    #[test]
    fn test_contrast_validation() {
        let system = AuroraColorSystem::new(ThemeName::Light);
        assert!(system.is_contrast_validated());
    }

    #[test]
    fn test_all_themes() {
        for theme in &[
            ThemeName::Light,
            ThemeName::Dark,
            ThemeName::OLED,
            ThemeName::HDR,
        ] {
            let system = AuroraColorSystem::new(*theme);
            assert_eq!(system.theme(), *theme);
        }
    }
}
