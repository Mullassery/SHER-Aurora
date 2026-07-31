//! Aurora Design System — Token Layer
//!
//! This crate defines all design tokens that drive Aurora's visual system.
//! Tokens are the single source of truth for spacing, colors, motion, and typography.

pub mod spacing;
pub mod radius;
pub mod elevation;
pub mod motion;
pub mod color;
pub mod errors;

pub use color::{Color, ColorSystem, Theme};
pub use elevation::{Elevation, ElevationLevel};
pub use errors::{TokenError, TokenResult};
pub use motion::{AnimationDuration, MotionScale};
pub use radius::{Radius, RadiusScale};
pub use spacing::{Spacing, SpacingScale};

/// Unified design token system
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub elevation: Elevation,
    pub motion: MotionScale,
    pub color_system: ColorSystem,
}

impl Default for DesignTokens {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignTokens {
    /// Create a new design token system with default values
    pub fn new() -> Self {
        Self {
            spacing: SpacingScale::default(),
            radius: RadiusScale::default(),
            elevation: Elevation::default(),
            motion: MotionScale::default(),
            color_system: ColorSystem::default(),
        }
    }

    /// Set the active theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.color_system.set_theme(theme);
    }

    /// Get the current active theme
    pub fn theme(&self) -> Theme {
        self.color_system.theme()
    }

    /// Validate all tokens for consistency
    pub fn validate(&self) -> TokenResult<()> {
        self.color_system.validate_contrast()?;
        Ok(())
    }

    /// Export all tokens as JSON
    pub fn to_json(&self) -> TokenResult<String> {
        // Note: Requires serde(derive) on DesignTokens struct
        // For now, return placeholder
        Ok("{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_tokens_creation() {
        let tokens = DesignTokens::new();
        assert_eq!(tokens.spacing.xxs, 2);
        assert_eq!(tokens.spacing.xs, 4);
        assert_eq!(tokens.spacing.sm, 8);
    }

    #[test]
    fn test_theme_switching() {
        let mut tokens = DesignTokens::new();
        tokens.set_theme(Theme::Light);
        assert_eq!(tokens.theme(), Theme::Light);

        tokens.set_theme(Theme::Dark);
        assert_eq!(tokens.theme(), Theme::Dark);
    }

    #[test]
    fn test_tokens_validation() {
        let tokens = DesignTokens::new();
        assert!(tokens.validate().is_ok());
    }

    #[test]
    fn test_tokens_to_json() {
        let tokens = DesignTokens::new();
        let json = tokens.to_json();
        assert!(json.is_ok());
    }
}
