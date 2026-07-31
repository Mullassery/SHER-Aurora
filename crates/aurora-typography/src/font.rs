use serde::{Deserialize, Serialize};

/// Font family preference
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FontFamily {
    /// Inter — modern, neutral sans-serif (primary)
    Inter,
    /// IBM Plex Sans — geometric, friendly (fallback 1)
    IBMPlexSans,
    /// Noto Sans — universal script support (fallback 2)
    NotoSans,
    /// Monospace for code/terminal
    Monospace,
}

impl FontFamily {
    pub fn name(&self) -> &'static str {
        match self {
            FontFamily::Inter => "Inter",
            FontFamily::IBMPlexSans => "IBM Plex Sans",
            FontFamily::NotoSans => "Noto Sans",
            FontFamily::Monospace => "IBM Plex Mono",
        }
    }

    pub fn css_name(&self) -> String {
        format!("'{}'", self.name())
    }

    pub fn url(&self) -> Option<&'static str> {
        match self {
            FontFamily::Inter => Some("https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700"),
            FontFamily::IBMPlexSans => Some("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600;700"),
            FontFamily::NotoSans => Some("https://fonts.googleapis.com/css2?family=Noto+Sans:wght@400;500;600;700"),
            FontFamily::Monospace => Some("https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500;600"),
        }
    }

    pub fn fallback_chain(&self) -> Vec<FontFamily> {
        match self {
            FontFamily::Inter => vec![
                FontFamily::Inter,
                FontFamily::IBMPlexSans,
                FontFamily::NotoSans,
            ],
            FontFamily::IBMPlexSans => vec![
                FontFamily::IBMPlexSans,
                FontFamily::Inter,
                FontFamily::NotoSans,
            ],
            FontFamily::NotoSans => vec![
                FontFamily::NotoSans,
                FontFamily::Inter,
                FontFamily::IBMPlexSans,
            ],
            FontFamily::Monospace => vec![
                FontFamily::Monospace,
            ],
        }
    }
}

/// Font weight (400–700)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum FontWeight {
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
}

impl FontWeight {
    pub fn value(&self) -> u16 {
        *self as u16
    }

    pub fn css(&self) -> String {
        self.value().to_string()
    }
}

/// Font variant (style, width, optical sizing)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FontVariant {
    /// Normal upright
    Normal,
    /// Italic/slanted (use sparingly)
    Italic,
}

impl FontVariant {
    pub fn css(&self) -> &'static str {
        match self {
            FontVariant::Normal => "normal",
            FontVariant::Italic => "italic",
        }
    }
}

/// Complete font definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Font {
    pub family: FontFamily,
    pub weights: Vec<FontWeight>,
    pub variants: Vec<FontVariant>,
    pub supports_variable: bool,
    pub supports_optical_sizing: bool,
    pub priority: u8, // 0 = highest priority (primary font)
}

impl Font {
    pub fn new(
        family: FontFamily,
        weights: Vec<FontWeight>,
        supports_variable: bool,
        supports_optical_sizing: bool,
        priority: u8,
    ) -> Self {
        Self {
            family,
            weights,
            variants: vec![FontVariant::Normal],
            supports_variable,
            supports_optical_sizing,
            priority,
        }
    }

    /// Inter — primary sans-serif for Aurora
    pub fn inter() -> Self {
        Self {
            family: FontFamily::Inter,
            weights: vec![
                FontWeight::Regular,
                FontWeight::Medium,
                FontWeight::SemiBold,
                FontWeight::Bold,
            ],
            variants: vec![FontVariant::Normal],
            supports_variable: true,
            supports_optical_sizing: true,
            priority: 0,
        }
    }

    /// IBM Plex Sans — fallback serif
    pub fn ibm_plex_sans() -> Self {
        Self {
            family: FontFamily::IBMPlexSans,
            weights: vec![
                FontWeight::Regular,
                FontWeight::Medium,
                FontWeight::SemiBold,
                FontWeight::Bold,
            ],
            variants: vec![FontVariant::Normal],
            supports_variable: true,
            supports_optical_sizing: false,
            priority: 1,
        }
    }

    /// Noto Sans — universal script support
    pub fn noto_sans() -> Self {
        Self {
            family: FontFamily::NotoSans,
            weights: vec![
                FontWeight::Regular,
                FontWeight::Medium,
                FontWeight::SemiBold,
                FontWeight::Bold,
            ],
            variants: vec![FontVariant::Normal],
            supports_variable: false,
            supports_optical_sizing: false,
            priority: 2,
        }
    }

    /// IBM Plex Mono — monospace for code
    pub fn fallback() -> Self {
        Self {
            family: FontFamily::Monospace,
            weights: vec![FontWeight::Regular, FontWeight::SemiBold],
            variants: vec![FontVariant::Normal],
            supports_variable: true,
            supports_optical_sizing: false,
            priority: 100, // System fallback
        }
    }

    pub fn css_import(&self) -> Option<String> {
        self.family.url().map(|url| {
            format!("@import url('{}');", url)
        })
    }

    pub fn has_weight(&self, weight: FontWeight) -> bool {
        self.weights.contains(&weight)
    }

    pub fn has_variant(&self, variant: FontVariant) -> bool {
        self.variants.contains(&variant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_family_names() {
        assert_eq!(FontFamily::Inter.name(), "Inter");
        assert_eq!(FontFamily::IBMPlexSans.name(), "IBM Plex Sans");
    }

    #[test]
    fn test_font_family_css_name() {
        assert_eq!(FontFamily::Inter.css_name(), "'Inter'");
    }

    #[test]
    fn test_font_weight_value() {
        assert_eq!(FontWeight::Regular.value(), 400);
        assert_eq!(FontWeight::Bold.value(), 700);
    }

    #[test]
    fn test_font_weight_css() {
        assert_eq!(FontWeight::SemiBold.css(), "600");
    }

    #[test]
    fn test_font_variant_css() {
        assert_eq!(FontVariant::Normal.css(), "normal");
        assert_eq!(FontVariant::Italic.css(), "italic");
    }

    #[test]
    fn test_font_inter() {
        let font = Font::inter();
        assert_eq!(font.family, FontFamily::Inter);
        assert!(font.supports_variable);
        assert!(font.supports_optical_sizing);
        assert_eq!(font.priority, 0);
    }

    #[test]
    fn test_font_has_weight() {
        let font = Font::inter();
        assert!(font.has_weight(FontWeight::Bold));
        assert!(font.has_weight(FontWeight::Regular));
    }

    #[test]
    fn test_font_fallback_chain() {
        let chain = FontFamily::Inter.fallback_chain();
        assert_eq!(chain[0], FontFamily::Inter);
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_font_css_import() {
        let font = Font::inter();
        assert!(font.css_import().is_some());
        assert!(font.css_import().unwrap().contains("@import"));
    }
}
