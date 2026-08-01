use crate::color::Color;
use crate::theme::ColorSystem;

/// Contrast ratio for WCAG compliance
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContrastRatio {
    /// Fails WCAG requirements
    Fail,
    /// Passes WCAG AA (4.5:1)
    AA,
    /// Passes WCAG AAA (7:1)
    AAA,
}

/// Validate contrast between text and background colors
pub fn validate_contrast(colors: &ColorSystem) -> bool {
    // Check primary text on primary surface
    if !colors.foreground.passes_wcag_aaa(&colors.background) {
        return false;
    }

    // Check primary color on surface
    if !colors.primary.passes_wcag_aaa(&colors.surface) {
        return false;
    }

    // Check error color on surface
    if !colors.error.passes_wcag_aaa(&colors.surface) {
        return false;
    }

    // Check success color on surface
    if !colors.success.passes_wcag_aaa(&colors.surface) {
        return false;
    }

    true
}

/// Check contrast ratio between two colors
pub fn check_contrast(foreground: &Color, background: &Color) -> ContrastRatio {
    let ratio = foreground.contrast_ratio(background);

    if ratio >= 7.0 {
        ContrastRatio::AAA
    } else if ratio >= 4.5 {
        ContrastRatio::AA
    } else {
        ContrastRatio::Fail
    }
}

/// Get contrast ratio value
pub fn get_contrast_ratio(foreground: &Color, background: &Color) -> f32 {
    foreground.contrast_ratio(background)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::ThemeName;

    #[test]
    fn test_validate_light_contrast() {
        let colors = ColorSystem::from_theme(ThemeName::Light);
        assert!(validate_contrast(&colors));
    }

    #[test]
    fn test_validate_dark_contrast() {
        let colors = ColorSystem::from_theme(ThemeName::Dark);
        assert!(validate_contrast(&colors));
    }

    #[test]
    fn test_check_contrast_aaa() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);

        let ratio = check_contrast(&black, &white);
        assert_eq!(ratio, ContrastRatio::AAA);
    }

    #[test]
    fn test_check_contrast_aa() {
        let light_gray = Color::new(200, 200, 200);
        let dark_gray = Color::new(50, 50, 50);

        let ratio = check_contrast(&light_gray, &dark_gray);
        assert!(matches!(ratio, ContrastRatio::AA | ContrastRatio::AAA));
    }

    #[test]
    fn test_get_contrast_ratio() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);

        let ratio = get_contrast_ratio(&black, &white);
        assert!(ratio > 20.0);
    }

    #[test]
    fn test_contrast_symmetry() {
        let color1 = Color::new(100, 100, 100);
        let color2 = Color::new(200, 200, 200);

        let ratio1 = get_contrast_ratio(&color1, &color2);
        let ratio2 = get_contrast_ratio(&color2, &color1);

        assert_eq!(ratio1, ratio2);
    }
}
