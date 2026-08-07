//! WCAG 2.1 contrast conformance levels.
//!
//! Implements the actual per-criterion thresholds instead of a single flat ratio:
//! normal text (SC 1.4.3/1.4.6), large text (>=18pt, or >=14pt bold), and non-text
//! UI components (SC 1.4.11, borders/icons/focus indicators).

use aurora_color::Color;

/// Text size class, since WCAG relaxes thresholds for large text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize {
    /// Body text below 18pt (or below 14pt bold).
    Normal,
    /// Text at or above 18pt, or at or above 14pt bold.
    Large,
}

/// Conformance level reached by a foreground/background pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WcagLevel {
    Fail,
    AA,
    AAA,
}

/// Determine the WCAG text-contrast level a pair reaches for the given text size.
pub fn wcag_level(foreground: &Color, background: &Color, size: TextSize) -> WcagLevel {
    let ratio = foreground.contrast_ratio(background);
    let (aa, aaa) = match size {
        TextSize::Normal => (4.5, 7.0),
        TextSize::Large => (3.0, 4.5),
    };
    if ratio >= aaa {
        WcagLevel::AAA
    } else if ratio >= aa {
        WcagLevel::AA
    } else {
        WcagLevel::Fail
    }
}

/// SC 1.4.11 Non-text Contrast: UI components (borders, icons, focus indicators) need
/// >=3:1 against their adjacent color. There is no separate AAA tier for this criterion.
pub fn passes_ui_component_contrast(foreground: &Color, background: &Color) -> bool {
    foreground.contrast_ratio(background) >= 3.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_text_requires_higher_ratio_than_large_text() {
        let white = Color::new(255, 255, 255);
        // ~4.54:1 - passes large-text AAA, fails normal-text AAA.
        let mid_gray = Color::new(0x76, 0x76, 0x76);
        assert_eq!(
            wcag_level(&mid_gray, &white, TextSize::Large),
            WcagLevel::AAA
        );
        assert_eq!(
            wcag_level(&mid_gray, &white, TextSize::Normal),
            WcagLevel::AA
        );
    }

    #[test]
    fn black_on_white_is_aaa_at_any_size() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        assert_eq!(wcag_level(&black, &white, TextSize::Normal), WcagLevel::AAA);
        assert_eq!(wcag_level(&black, &white, TextSize::Large), WcagLevel::AAA);
    }

    #[test]
    fn low_contrast_pair_fails_even_large_text() {
        let white = Color::new(255, 255, 255);
        let near_white = Color::new(0xF0, 0xF0, 0xF0);
        assert_eq!(
            wcag_level(&near_white, &white, TextSize::Large),
            WcagLevel::Fail
        );
    }

    #[test]
    fn ui_component_contrast_matches_1_4_11_threshold() {
        let bg = Color::new(255, 255, 255);
        let passing = Color::new(0x5A, 0x5A, 0x5A); // >=3:1 against white
        let failing = Color::new(0xE0, 0xE0, 0xE0); // <3:1 against white
        assert!(passes_ui_component_contrast(&passing, &bg));
        assert!(!passes_ui_component_contrast(&failing, &bg));
    }
}
