/// Aurora Color representation
///
/// Represents a color in RGB format (0-255 for each component).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create color from hex string (e.g., "#FF0000")
    pub fn from_hex(hex: &str) -> Option<Self> {
        if !hex.starts_with('#') || hex.len() != 7 {
            return None;
        }

        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

        Some(Self { r, g, b })
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Convert to RGB string
    pub fn to_rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    /// Calculate luminance for contrast calculations
    pub fn luminance(&self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let r = if r <= 0.03928 {
            r / 12.92
        } else {
            ((r + 0.055) / 1.055).powf(2.4)
        };

        let g = if g <= 0.03928 {
            g / 12.92
        } else {
            ((g + 0.055) / 1.055).powf(2.4)
        };

        let b = if b <= 0.03928 {
            b / 12.92
        } else {
            ((b + 0.055) / 1.055).powf(2.4)
        };

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.luminance();
        let l2 = other.luminance();

        let lighter = l1.max(l2);
        let darker = l1.min(l2);

        (lighter + 0.05) / (darker + 0.05)
    }

    /// Check if passes WCAG AAA contrast for normal-size text (7:1)
    pub fn passes_wcag_aaa(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 7.0
    }

    /// Check if passes WCAG AA contrast for normal-size text (4.5:1)
    pub fn passes_wcag_aa(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 4.5
    }

    /// Check if passes WCAG AAA contrast for large text (>=18pt, or >=14pt bold): 4.5:1.
    /// WCAG 2.1 SC 1.4.6 relaxes the AAA threshold for large text vs. normal text.
    pub fn passes_wcag_aaa_large(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 4.5
    }

    /// Check if passes WCAG AA contrast for large text (>=18pt, or >=14pt bold): 3:1.
    /// WCAG 2.1 SC 1.4.3 relaxes the AA threshold for large text vs. normal text.
    pub fn passes_wcag_aa_large(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 3.0
    }

    /// Check if passes WCAG 1.4.11 non-text contrast for UI components (icons, borders,
    /// focus indicators): 3:1. This criterion has no separate AAA tier in the spec.
    pub fn passes_wcag_ui_component(&self, other: &Color) -> bool {
        self.contrast_ratio(other) >= 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(255, 0, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color::new(255, 0, 0);
        assert_eq!(color.to_hex(), "#ff0000");
    }

    #[test]
    fn test_color_to_rgb() {
        let color = Color::new(255, 0, 0);
        assert_eq!(color.to_rgb(), "rgb(255, 0, 0)");
    }

    #[test]
    fn test_color_luminance() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);

        assert!(white.luminance() > black.luminance());
    }

    #[test]
    fn test_color_contrast_ratio() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);

        let ratio = white.contrast_ratio(&black);
        assert!(ratio > 20.0); // Maximum contrast
    }

    #[test]
    fn test_wcag_aaa_contrast() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);

        assert!(white.passes_wcag_aaa(&black));
    }

    #[test]
    fn test_wcag_aa_contrast() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);

        assert!(white.passes_wcag_aa(&black));
    }

    #[test]
    fn test_color_equality() {
        let color1 = Color::new(255, 0, 0);
        let color2 = Color::new(255, 0, 0);
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_color_default() {
        let color = Color::default();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_wcag_aaa_large_text_threshold_is_lower_than_normal() {
        // A pair that clears large-text AAA (4.5:1) but not normal-text AAA (7:1).
        let fg = Color::new(0x76, 0x76, 0x76); // #767676 on white is ~4.54:1
        let bg = Color::new(0xFF, 0xFF, 0xFF);
        let ratio = fg.contrast_ratio(&bg);
        assert!(
            (4.5..7.0).contains(&ratio),
            "fixture ratio out of expected band: {ratio}"
        );
        assert!(fg.passes_wcag_aaa_large(&bg));
        assert!(!fg.passes_wcag_aaa(&bg));
    }

    #[test]
    fn test_wcag_aa_large_text_threshold_is_lower_than_aaa_large() {
        // A pair that clears large-text AA (3:1) but not large-text AAA (4.5:1).
        let fg = Color::new(0x92, 0x92, 0x92); // #929292 on white is ~3.11:1
        let bg = Color::new(0xFF, 0xFF, 0xFF);
        let ratio = fg.contrast_ratio(&bg);
        assert!(
            (3.0..4.5).contains(&ratio),
            "fixture ratio out of expected band: {ratio}"
        );
        assert!(fg.passes_wcag_aa_large(&bg));
        assert!(!fg.passes_wcag_aaa_large(&bg));
    }

    #[test]
    fn test_wcag_ui_component_contrast() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        assert!(black.passes_wcag_ui_component(&white));

        // A pair below the 3:1 non-text threshold must fail.
        let light_gray = Color::new(0xE0, 0xE0, 0xE0);
        assert!(!light_gray.passes_wcag_ui_component(&white));
    }
}
