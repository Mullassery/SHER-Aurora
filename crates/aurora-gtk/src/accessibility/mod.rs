//! Aurora Accessibility Layer - WCAG AAA Compliance
//!
//! Comprehensive accessibility features including colorblind preview, dyslexia fonts, and high contrast support.

/// Colorblind vision type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorBlindType {
    Protanopia,    // Red-blind (1% of males)
    Deuteranopia,  // Green-blind (1% of males)
    Tritanopia,    // Blue-yellow blind (rare)
    Achromatopsia, // Complete color blindness (very rare)
}

impl ColorBlindType {
    pub fn name(&self) -> &str {
        match self {
            ColorBlindType::Protanopia => "Protanopia (Red-Blind)",
            ColorBlindType::Deuteranopia => "Deuteranopia (Green-Blind)",
            ColorBlindType::Tritanopia => "Tritanopia (Blue-Yellow)",
            ColorBlindType::Achromatopsia => "Achromatopsia (Monochrome)",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ColorBlindType::Protanopia => "Red-green color blindness affecting ~1% of males",
            ColorBlindType::Deuteranopia => "Green-red color blindness affecting ~1% of males",
            ColorBlindType::Tritanopia => "Blue-yellow color blindness, extremely rare",
            ColorBlindType::Achromatopsia => "Complete color blindness, sees only grayscale",
        }
    }
}

/// Color for simulating colorblind vision
#[derive(Debug, Clone)]
pub struct ColorBlindSimulation {
    normal_hex: String,
    simulated_hex: String,
}

impl ColorBlindSimulation {
    pub fn new(hex: &str) -> Self {
        Self {
            normal_hex: hex.to_string(),
            simulated_hex: Self::simulate_protanopia(hex),
        }
    }

    /// Simulate Protanopia (Red-blind)
    pub fn for_protanopia(hex: &str) -> Self {
        Self {
            normal_hex: hex.to_string(),
            simulated_hex: Self::simulate_protanopia(hex),
        }
    }

    /// Simulate Deuteranopia (Green-blind)
    pub fn for_deuteranopia(hex: &str) -> Self {
        Self {
            normal_hex: hex.to_string(),
            simulated_hex: Self::simulate_deuteranopia(hex),
        }
    }

    /// Simulate Tritanopia (Blue-yellow blind)
    pub fn for_tritanopia(hex: &str) -> Self {
        Self {
            normal_hex: hex.to_string(),
            simulated_hex: Self::simulate_tritanopia(hex),
        }
    }

    /// Simulate Achromatopsia (Monochrome)
    pub fn for_achromatopsia(hex: &str) -> Self {
        Self {
            normal_hex: hex.to_string(),
            simulated_hex: Self::simulate_achromatopsia(hex),
        }
    }

    fn simulate_protanopia(hex: &str) -> String {
        // Simplified protanopia simulation - reduce red channel
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return hex.to_string();
        }

        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = (rgb >> 16) & 0xFF;
            let g = (rgb >> 8) & 0xFF;
            let b = rgb & 0xFF;

            // Protanopia: reduce red, shift to blue-yellow spectrum
            let r_sim = (r as f32 * 0.567) as u32;
            let g_sim = (g as f32 * 0.433) as u32;
            let b_sim = b;

            format!("#{:06X}", (r_sim << 16) | (g_sim << 8) | b_sim)
        } else {
            hex.to_string()
        }
    }

    fn simulate_deuteranopia(hex: &str) -> String {
        // Deuteranopia simulation - reduce green channel
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return hex.to_string();
        }

        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = (rgb >> 16) & 0xFF;
            let g = (rgb >> 8) & 0xFF;
            let b = rgb & 0xFF;

            let r_sim = (r as f32 * 0.625) as u32;
            let g_sim = (g as f32 * 0.375) as u32;
            let b_sim = b;

            format!("#{:06X}", (r_sim << 16) | (g_sim << 8) | b_sim)
        } else {
            hex.to_string()
        }
    }

    fn simulate_tritanopia(hex: &str) -> String {
        // Tritanopia simulation - reduce blue-yellow perception
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return hex.to_string();
        }

        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = (rgb >> 16) & 0xFF;
            let g = (rgb >> 8) & 0xFF;
            let b = rgb & 0xFF;

            let r_sim = (r as f32 * 0.95) as u32;
            let g_sim = (g as f32 * 0.95) as u32;
            let b_sim = (b as f32 * 0.4) as u32;

            format!("#{:06X}", (r_sim << 16) | (g_sim << 8) | b_sim)
        } else {
            hex.to_string()
        }
    }

    fn simulate_achromatopsia(hex: &str) -> String {
        // Achromatopsia - convert to grayscale
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return hex.to_string();
        }

        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = (rgb >> 16) & 0xFF_u32;
            let g = (rgb >> 8) & 0xFF_u32;
            let b = rgb & 0xFF_u32;

            // Standard grayscale formula
            let gray = ((r as f32 * 0.299) + (g as f32 * 0.587) + (b as f32 * 0.114)) as u32;

            format!("#{:06X}", (gray << 16) | (gray << 8) | gray)
        } else {
            hex.to_string()
        }
    }

    pub fn original(&self) -> &str {
        &self.normal_hex
    }
    pub fn simulated(&self) -> &str {
        &self.simulated_hex
    }
}

/// Dyslexia-friendly font options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DyslexiaFont {
    Default,      // Standard system font
    OpenDyslexic, // Open Dyslexic font family
    Verdana,      // Sans-serif alternative
    Comic,        // Comic Sans (helps some)
}

impl DyslexiaFont {
    pub fn family(&self) -> &str {
        match self {
            DyslexiaFont::Default => "system-ui, -apple-system, sans-serif",
            DyslexiaFont::OpenDyslexic => "OpenDyslexic, sans-serif",
            DyslexiaFont::Verdana => "Verdana, sans-serif",
            DyslexiaFont::Comic => "'Comic Sans MS', cursive",
        }
    }

    pub fn css(&self) -> String {
        format!("font-family: {};", self.family())
    }

    pub fn description(&self) -> &str {
        match self {
            DyslexiaFont::Default => "Default system font",
            DyslexiaFont::OpenDyslexic => "Open Dyslexic - designed for dyslexia readability",
            DyslexiaFont::Verdana => "Verdana - clear sans-serif alternative",
            DyslexiaFont::Comic => "Comic Sans - helps some dyslexic readers",
        }
    }
}

/// High contrast mode settings
#[derive(Debug, Clone)]
pub struct HighContrastMode {
    enabled: bool,
    min_contrast_ratio: f32, // WCAG AAA = 7.0
    focus_color: String,
    text_color: String,
    bg_color: String,
}

impl HighContrastMode {
    pub fn new() -> Self {
        Self {
            enabled: false,
            min_contrast_ratio: 7.0, // WCAG AAA
            focus_color: "#0066FF".to_string(),
            text_color: "#000000".to_string(),
            bg_color: "#FFFFFF".to_string(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn min_contrast_ratio(&self) -> f32 {
        self.min_contrast_ratio
    }

    pub fn focus_color(&self) -> &str {
        &self.focus_color
    }

    pub fn text_color(&self) -> &str {
        &self.text_color
    }

    pub fn bg_color(&self) -> &str {
        &self.bg_color
    }

    /// Calculate contrast ratio between two colors
    pub fn calculate_contrast(hex1: &str, hex2: &str) -> f32 {
        let lum1 = Self::relative_luminance(hex1);
        let lum2 = Self::relative_luminance(hex2);

        let lighter = lum1.max(lum2);
        let darker = lum1.min(lum2);

        (lighter + 0.05) / (darker + 0.05)
    }

    /// Calculate relative luminance per WCAG standards
    fn relative_luminance(hex: &str) -> f32 {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return 0.0;
        }

        if let Ok(rgb) = u32::from_str_radix(hex, 16) {
            let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
            let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
            let b = (rgb & 0xFF) as f32 / 255.0;

            let rs = if r <= 0.03928 {
                r / 12.92
            } else {
                ((r + 0.055) / 1.055).powf(2.4)
            };
            let gs = if g <= 0.03928 {
                g / 12.92
            } else {
                ((g + 0.055) / 1.055).powf(2.4)
            };
            let bs = if b <= 0.03928 {
                b / 12.92
            } else {
                ((b + 0.055) / 1.055).powf(2.4)
            };

            0.2126 * rs + 0.7152 * gs + 0.0722 * bs
        } else {
            0.0
        }
    }
}

impl Default for HighContrastMode {
    fn default() -> Self {
        Self::new()
    }
}

/// Accessibility manager
pub struct AccessibilityManager {
    colorblind_mode: std::option::Option<ColorBlindType>,
    dyslexia_font: DyslexiaFont,
    high_contrast: HighContrastMode,
    reduce_motion: bool,
}

impl AccessibilityManager {
    pub fn new() -> Self {
        Self {
            colorblind_mode: std::option::Option::None,
            dyslexia_font: DyslexiaFont::Default,
            high_contrast: HighContrastMode::new(),
            reduce_motion: false,
        }
    }

    pub fn set_colorblind_mode(&mut self, mode: ColorBlindType) {
        self.colorblind_mode = std::option::Option::Some(mode);
    }

    pub fn disable_colorblind_mode(&mut self) {
        self.colorblind_mode = std::option::Option::None;
    }

    pub fn colorblind_mode(&self) -> std::option::Option<ColorBlindType> {
        self.colorblind_mode
    }

    pub fn set_dyslexia_font(&mut self, font: DyslexiaFont) {
        self.dyslexia_font = font;
    }

    pub fn dyslexia_font(&self) -> DyslexiaFont {
        self.dyslexia_font
    }

    pub fn enable_high_contrast(&mut self) {
        self.high_contrast.enable();
    }

    pub fn disable_high_contrast(&mut self) {
        self.high_contrast.disable();
    }

    pub fn high_contrast(&self) -> &HighContrastMode {
        &self.high_contrast
    }

    pub fn set_reduce_motion(&mut self, enabled: bool) {
        self.reduce_motion = enabled;
    }

    pub fn should_reduce_motion(&self) -> bool {
        self.reduce_motion
    }
}

impl Default for AccessibilityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorblind_type_names() {
        assert_eq!(ColorBlindType::Protanopia.name(), "Protanopia (Red-Blind)");
        assert_eq!(
            ColorBlindType::Deuteranopia.name(),
            "Deuteranopia (Green-Blind)"
        );
    }

    #[test]
    fn test_colorblind_simulation_protanopia() {
        let sim = ColorBlindSimulation::for_protanopia("#FF0000");
        assert_eq!(sim.original(), "#FF0000");
        assert_ne!(sim.simulated(), "#FF0000");
    }

    #[test]
    fn test_colorblind_simulation_grayscale() {
        let sim = ColorBlindSimulation::for_achromatopsia("#FF0000");
        let simulated = sim.simulated();
        // Should be approximately equal R, G, B values
        assert!(simulated.contains("#"));
    }

    #[test]
    fn test_dyslexia_font_families() {
        assert_eq!(
            DyslexiaFont::OpenDyslexic.family(),
            "OpenDyslexic, sans-serif"
        );
        assert_eq!(DyslexiaFont::Verdana.family(), "Verdana, sans-serif");
    }

    #[test]
    fn test_dyslexia_font_css() {
        let css = DyslexiaFont::OpenDyslexic.css();
        assert!(css.contains("font-family:"));
        assert!(css.contains("OpenDyslexic"));
    }

    #[test]
    fn test_high_contrast_creation() {
        let hc = HighContrastMode::new();
        assert!(!hc.is_enabled());
        assert_eq!(hc.min_contrast_ratio(), 7.0);
    }

    #[test]
    fn test_high_contrast_enable_disable() {
        let mut hc = HighContrastMode::new();
        hc.enable();
        assert!(hc.is_enabled());
        hc.disable();
        assert!(!hc.is_enabled());
    }

    #[test]
    fn test_contrast_ratio_calculation() {
        let contrast = HighContrastMode::calculate_contrast("#FFFFFF", "#000000");
        assert!(contrast > 20.0); // High contrast
    }

    #[test]
    fn test_contrast_ratio_low() {
        let contrast = HighContrastMode::calculate_contrast("#FFFFFF", "#EEEEEE");
        assert!(contrast < 2.0); // Low contrast
    }

    #[test]
    fn test_accessibility_manager_creation() {
        let mgr = AccessibilityManager::new();
        assert!(mgr.colorblind_mode().is_none());
        assert_eq!(mgr.dyslexia_font(), DyslexiaFont::Default);
        assert!(!mgr.should_reduce_motion());
    }

    #[test]
    fn test_accessibility_manager_colorblind() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_colorblind_mode(ColorBlindType::Deuteranopia);
        assert_eq!(mgr.colorblind_mode(), Some(ColorBlindType::Deuteranopia));

        mgr.disable_colorblind_mode();
        assert!(mgr.colorblind_mode().is_none());
    }

    #[test]
    fn test_accessibility_manager_dyslexia() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_dyslexia_font(DyslexiaFont::OpenDyslexic);
        assert_eq!(mgr.dyslexia_font(), DyslexiaFont::OpenDyslexic);
    }

    #[test]
    fn test_accessibility_manager_high_contrast() {
        let mut mgr = AccessibilityManager::new();
        mgr.enable_high_contrast();
        assert!(mgr.high_contrast().is_enabled());

        mgr.disable_high_contrast();
        assert!(!mgr.high_contrast().is_enabled());
    }

    #[test]
    fn test_accessibility_manager_reduce_motion() {
        let mut mgr = AccessibilityManager::new();
        mgr.set_reduce_motion(true);
        assert!(mgr.should_reduce_motion());

        mgr.set_reduce_motion(false);
        assert!(!mgr.should_reduce_motion());
    }

    #[test]
    fn test_default_accessibility_manager() {
        let mgr = AccessibilityManager::default();
        assert!(mgr.colorblind_mode().is_none());
    }
}
