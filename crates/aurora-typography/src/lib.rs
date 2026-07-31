//! Aurora Typography Engine
//!
//! Unified typography system with:
//! - Type scales (Display, Headline, Title, Body, Caption, Micro)
//! - Responsive typography across device sizes
//! - i18n support (CJK, RTL, complex scripts)
//! - Variable font support
//! - Optical sizing
//! - Line height adjustments per script family

pub mod errors;
pub mod font;
pub mod scale;
pub mod responsive;
pub mod script;

pub use errors::{TypographyError, TypographyResult};
pub use font::{Font, FontFamily, FontWeight, FontVariant};
pub use scale::{TypeScale, TypographyStyle, TextLevel};
pub use responsive::{ResponsiveTypography, Breakpoint, ViewportSize};
pub use script::{Script, ScriptAdjustment};

/// Unified typography system
#[derive(Debug, Clone)]
pub struct Typography {
    pub type_scale: TypeScale,
    pub responsive: ResponsiveTypography,
    pub fonts: Vec<Font>,
}

impl Default for Typography {
    fn default() -> Self {
        Self::new()
    }
}

impl Typography {
    /// Create a new typography system with default scales
    pub fn new() -> Self {
        Self {
            type_scale: TypeScale::default(),
            responsive: ResponsiveTypography::default(),
            fonts: vec![Font::inter(), Font::fallback()],
        }
    }

    /// Get the typography style for a text level at a given viewport
    pub fn get_style(&self, level: TextLevel, viewport: ViewportSize) -> TypographyStyle {
        let mut style = self.type_scale.get_style(level);
        self.responsive.apply_responsive_adjustments(&mut style, viewport);
        style
    }

    /// Adjust typography for a specific script (CJK, RTL, etc.)
    pub fn adjust_for_script(&self, style: &mut TypographyStyle, script: Script) {
        let adjustment = script.adjustment();
        style.line_height = adjustment.line_height_multiplier * style.line_height;
    }

    /// Get optimal line length for a script in characters
    pub fn optimal_line_length(&self, script: Script) -> u16 {
        script.optimal_line_length()
    }

    /// Validate typography consistency
    pub fn validate(&self) -> TypographyResult<()> {
        self.type_scale.validate()?;
        self.responsive.validate()?;
        Ok(())
    }

    /// Export all typography styles as JSON
    pub fn to_json(&self) -> TypographyResult<String> {
        serde_json::to_string_pretty(&self.type_scale)
            .map_err(|e| TypographyError::SerializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typography_creation() {
        let typography = Typography::new();
        assert_eq!(typography.fonts.len(), 2);
    }

    #[test]
    fn test_get_style_at_viewport() {
        let typography = Typography::new();
        let style = typography.get_style(TextLevel::Body, ViewportSize::new(1920, 1080));
        assert!(style.font_size > 0);
    }

    #[test]
    fn test_script_adjustment() {
        let typography = Typography::new();
        let mut style = typography.type_scale.get_style(TextLevel::Body);
        let original_line_height = style.line_height;
        typography.adjust_for_script(&mut style, Script::CJK);
        assert!(style.line_height > original_line_height);
    }

    #[test]
    fn test_optimal_line_length() {
        let typography = Typography::new();
        let latin_length = typography.optimal_line_length(Script::Latin);
        let cjk_length = typography.optimal_line_length(Script::CJK);
        assert!(latin_length > cjk_length); // Latin needs more chars, CJK needs fewer
    }

    #[test]
    fn test_typography_validation() {
        let typography = Typography::new();
        assert!(typography.validate().is_ok());
    }

    #[test]
    fn test_typography_to_json() {
        let typography = Typography::new();
        let json = typography.to_json();
        assert!(json.is_ok());
    }
}
