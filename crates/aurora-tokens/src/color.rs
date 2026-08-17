use crate::errors::{TokenError, TokenResult};
use serde::{Deserialize, Serialize};

/// Theme identifier
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    OLED,
    HDR,
}

/// RGBA color representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> TokenResult<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() < 6 {
            return Err(TokenError::InvalidValue(
                "Hex color must be 6 or 8 characters".to_string(),
            ));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| TokenError::InvalidValue("Invalid hex color".to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| TokenError::InvalidValue("Invalid hex color".to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| TokenError::InvalidValue("Invalid hex color".to_string()))?;

        let a = if hex.len() >= 8 {
            let alpha = u8::from_str_radix(&hex[6..8], 16)
                .map_err(|_| TokenError::InvalidValue("Invalid hex alpha".to_string()))?;
            alpha as f32 / 255.0
        } else {
            1.0
        };

        Ok(Self { r, g, b, a })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn to_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }

    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// Calculate perceived luminance (relative luminance) for contrast ratio
    fn relative_luminance(&self) -> f32 {
        let r = (self.r as f32 / 255.0).powf(2.2);
        let g = (self.g as f32 / 255.0).powf(2.2);
        let b = (self.b as f32 / 255.0).powf(2.2);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Calculate contrast ratio between two colors (WCAG)
    pub fn contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.relative_luminance();
        let l2 = other.relative_luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }
}

/// Semantic color token
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SemanticColor {
    pub surface: Color,
    pub surface_variant: Color,
    pub surface_inverse: Color,
    pub background: Color,
    pub background_secondary: Color,
    pub foreground: Color,
    pub foreground_secondary: Color,
    pub foreground_tertiary: Color,
    pub foreground_inverse: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub outline: Color,
}

impl SemanticColor {
    pub fn light() -> Self {
        Self {
            surface: Color::from_hex("#f5f5f5").unwrap(),
            surface_variant: Color::from_hex("#efefef").unwrap(),
            surface_inverse: Color::from_hex("#1a1a1a").unwrap(),
            background: Color::from_hex("#ffffff").unwrap(),
            background_secondary: Color::from_hex("#f9f9f9").unwrap(),
            foreground: Color::from_hex("#1a1a1a").unwrap(),
            foreground_secondary: Color::from_hex("#616161").unwrap(),
            foreground_tertiary: Color::from_hex("#9e9e9e").unwrap(),
            foreground_inverse: Color::from_hex("#ffffff").unwrap(),
            primary: Color::from_hex("#0066cc").unwrap(),
            secondary: Color::from_hex("#6200ee").unwrap(),
            accent: Color::from_hex("#ff4081").unwrap(),
            success: Color::from_hex("#4caf50").unwrap(),
            warning: Color::from_hex("#ffc107").unwrap(),
            error: Color::from_hex("#f44336").unwrap(),
            info: Color::from_hex("#2196f3").unwrap(),
            outline: Color::from_hex("#e0e0e0").unwrap(),
        }
    }

    pub fn dark() -> Self {
        Self {
            surface: Color::from_hex("#1e1e1e").unwrap(),
            surface_variant: Color::from_hex("#2a2a2a").unwrap(),
            surface_inverse: Color::from_hex("#f5f5f5").unwrap(),
            background: Color::from_hex("#121212").unwrap(),
            background_secondary: Color::from_hex("#1a1a1a").unwrap(),
            foreground: Color::from_hex("#f5f5f5").unwrap(),
            foreground_secondary: Color::from_hex("#b3b3b3").unwrap(),
            foreground_tertiary: Color::from_hex("#757575").unwrap(),
            foreground_inverse: Color::from_hex("#1a1a1a").unwrap(),
            primary: Color::from_hex("#6eb7ff").unwrap(),
            secondary: Color::from_hex("#c5b3ff").unwrap(),
            accent: Color::from_hex("#ff80ab").unwrap(),
            success: Color::from_hex("#81c784").unwrap(),
            warning: Color::from_hex("#ffca28").unwrap(),
            error: Color::from_hex("#ef5350").unwrap(),
            info: Color::from_hex("#64b5f6").unwrap(),
            outline: Color::from_hex("#424242").unwrap(),
        }
    }

    pub fn oled() -> Self {
        Self {
            surface: Color::from_hex("#0d0d0d").unwrap(),
            surface_variant: Color::from_hex("#1a1a1a").unwrap(),
            surface_inverse: Color::from_hex("#f5f5f5").unwrap(),
            background: Color::from_hex("#000000").unwrap(),
            background_secondary: Color::from_hex("#0d0d0d").unwrap(),
            foreground: Color::from_hex("#f5f5f5").unwrap(),
            foreground_secondary: Color::from_hex("#b3b3b3").unwrap(),
            foreground_tertiary: Color::from_hex("#757575").unwrap(),
            foreground_inverse: Color::from_hex("#000000").unwrap(),
            primary: Color::from_hex("#6eb7ff").unwrap(),
            secondary: Color::from_hex("#c5b3ff").unwrap(),
            accent: Color::from_hex("#ff80ab").unwrap(),
            success: Color::from_hex("#81c784").unwrap(),
            warning: Color::from_hex("#ffca28").unwrap(),
            error: Color::from_hex("#ef5350").unwrap(),
            info: Color::from_hex("#64b5f6").unwrap(),
            outline: Color::from_hex("#333333").unwrap(),
        }
    }

    /// Validate contrast ratios (WCAG AAA)
    pub fn validate_contrast(&self) -> TokenResult<()> {
        let checks = vec![
            (
                "foreground on background",
                self.foreground,
                self.background,
                7.0,
            ),
            (
                "foreground_secondary on background",
                self.foreground_secondary,
                self.background,
                4.5,
            ),
            ("primary on surface", self.primary, self.surface, 3.0),
            ("error on background", self.error, self.background, 3.0),
        ];

        for (desc, fg, bg, min_ratio) in checks {
            let ratio = fg.contrast_ratio(&bg);
            if ratio < min_ratio {
                return Err(TokenError::ContrastRatioError(format!(
                    "{}: {:.2} < {:.2}",
                    desc, ratio, min_ratio
                )));
            }
        }

        Ok(())
    }
}

/// Color system with theme management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSystem {
    light: SemanticColor,
    dark: SemanticColor,
    oled: SemanticColor,
    active_theme: Theme,
}

impl Default for ColorSystem {
    fn default() -> Self {
        Self::new(Theme::Light)
    }
}

impl ColorSystem {
    pub fn new(theme: Theme) -> Self {
        Self {
            light: SemanticColor::light(),
            dark: SemanticColor::dark(),
            oled: SemanticColor::oled(),
            active_theme: theme,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.active_theme = theme;
    }

    pub fn theme(&self) -> Theme {
        self.active_theme
    }

    pub fn current(&self) -> &SemanticColor {
        match self.active_theme {
            Theme::Light => &self.light,
            Theme::Dark => &self.dark,
            Theme::OLED => &self.oled,
            Theme::HDR => &self.light, // TODO: Implement HDR theme
        }
    }

    pub fn validate_contrast(&self) -> TokenResult<()> {
        self.light.validate_contrast()?;
        self.dark.validate_contrast()?;
        self.oled.validate_contrast()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 0, 0, 1.0);
        assert_eq!(color.to_hex(), "#ff0000");
    }

    #[test]
    fn test_color_to_rgba() {
        let color = Color::new(255, 0, 0, 0.5);
        assert_eq!(color.to_rgba(), "rgba(255, 0, 0, 0.5)");
    }

    #[test]
    fn test_contrast_ratio() {
        let white = Color::new(255, 255, 255, 1.0);
        let black = Color::new(0, 0, 0, 1.0);
        let ratio = white.contrast_ratio(&black);
        assert!(ratio > 20.0); // Should be 21:1
    }

    #[test]
    fn test_semantic_color_light() {
        let colors = SemanticColor::light();
        assert_eq!(colors.background.to_hex(), "#ffffff");
        assert_eq!(colors.foreground.to_hex(), "#1a1a1a");
    }

    #[test]
    fn test_color_system_theme_switching() {
        let mut system = ColorSystem::new(Theme::Light);
        assert_eq!(system.theme(), Theme::Light);
        system.set_theme(Theme::Dark);
        assert_eq!(system.theme(), Theme::Dark);
    }

    #[test]
    fn test_semantic_color_validation() {
        let colors = SemanticColor::light();
        assert!(colors.validate_contrast().is_ok());
    }
}
