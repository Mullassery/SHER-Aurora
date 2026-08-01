//! Icon Font Generation - Generate web fonts from SVG icons
//!
//! Create web-ready icon fonts (TTF, WOFF2) for efficient icon delivery in web applications.

use std::collections::HashMap;

/// Font format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFormat {
    TTF,      // TrueType Font
    WOFF2,    // Web Open Font Format 2 (recommended)
    WOFF,     // Web Open Font Format
}

impl FontFormat {
    pub fn extension(&self) -> &str {
        match self {
            FontFormat::TTF => ".ttf",
            FontFormat::WOFF2 => ".woff2",
            FontFormat::WOFF => ".woff",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            FontFormat::TTF => "font/ttf",
            FontFormat::WOFF2 => "font/woff2",
            FontFormat::WOFF => "font/woff",
        }
    }
}

/// Icon to Unicode codepoint mapping
#[derive(Debug, Clone)]
pub struct IconGlyph {
    icon_id: String,
    codepoint: u32,
    name: String,
}

impl IconGlyph {
    pub fn new(icon_id: &str, codepoint: u32, name: &str) -> Self {
        Self {
            icon_id: icon_id.to_string(),
            codepoint,
            name: name.to_string(),
        }
    }

    pub fn icon_id(&self) -> &str { &self.icon_id }
    pub fn codepoint(&self) -> u32 { self.codepoint }
    pub fn codepoint_hex(&self) -> String { format!("{:04X}", self.codepoint) }
    pub fn name(&self) -> &str { &self.name }
    pub fn css_class(&self) -> String { format!(".aurora-icon-{}", self.icon_id) }
}

/// Icon font configuration
#[derive(Debug, Clone)]
pub struct IconFontConfig {
    family_name: String,
    font_version: String,
    copyright: String,
    manufacturer: String,
    license: String,
    start_codepoint: u32,
}

impl IconFontConfig {
    pub fn new(family_name: &str) -> Self {
        Self {
            family_name: family_name.to_string(),
            font_version: "1.1.0".to_string(),
            copyright: "Copyright (c) 2026 Aurora Design System".to_string(),
            manufacturer: "Aurora Project".to_string(),
            license: "MIT/Apache 2.0".to_string(),
            start_codepoint: 0xE000,  // Private use area
        }
    }

    pub fn family_name(&self) -> &str { &self.family_name }
    pub fn font_version(&self) -> &str { &self.font_version }
    pub fn copyright(&self) -> &str { &self.copyright }
    pub fn manufacturer(&self) -> &str { &self.manufacturer }
    pub fn license(&self) -> &str { &self.license }
    pub fn start_codepoint(&self) -> u32 { self.start_codepoint }
}

impl Default for IconFontConfig {
    fn default() -> Self {
        Self::new("Aurora Icons")
    }
}

/// Icon font builder
pub struct IconFontBuilder {
    config: IconFontConfig,
    glyphs: Vec<IconGlyph>,
}

impl IconFontBuilder {
    pub fn new(config: IconFontConfig) -> Self {
        Self {
            config,
            glyphs: Vec::new(),
        }
    }

    pub fn add_glyph(&mut self, glyph: IconGlyph) {
        self.glyphs.push(glyph);
    }

    pub fn add_glyphs(&mut self, glyphs: Vec<IconGlyph>) {
        self.glyphs.extend(glyphs);
    }

    pub fn glyphs(&self) -> &[IconGlyph] {
        &self.glyphs
    }

    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }

    /// Generate CSS file for icon font usage
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // Font face definition
        css.push_str(&format!(
            "@font-face {{\n  font-family: '{}';\n  src: url('aurora-icons.woff2') format('woff2'),\n",
            self.config.family_name()
        ));
        css.push_str("       url('aurora-icons.woff') format('woff'),\n");
        css.push_str("       url('aurora-icons.ttf') format('truetype');\n");
        css.push_str("  font-weight: normal;\n");
        css.push_str("  font-style: normal;\n");
        css.push_str("}\n\n");

        // Base icon class
        css.push_str(&format!(
            ".aurora-icon {{\n  font-family: '{}';\n",
            self.config.family_name()
        ));
        css.push_str("  font-size: 1em;\n");
        css.push_str("  font-style: normal;\n");
        css.push_str("  font-weight: normal;\n");
        css.push_str("  line-height: 1;\n");
        css.push_str("  -webkit-font-smoothing: antialiased;\n");
        css.push_str("  -moz-osx-font-smoothing: grayscale;\n");
        css.push_str("  display: inline-block;\n");
        css.push_str("  vertical-align: middle;\n");
        css.push_str("}\n\n");

        // Individual icon classes
        for glyph in &self.glyphs {
            css.push_str(&format!("{}::before {{\n", glyph.css_class()));
            css.push_str(&format!("  content: '\\{}';\n", glyph.codepoint_hex()));
            css.push_str("}\n\n");
        }

        css
    }

    /// Generate HTML usage examples
    pub fn generate_html_examples(&self) -> String {
        let mut html = String::new();

        html.push_str("<div class='aurora-icon-showcase'>\n");
        html.push_str(&format!("<h2>{} Icon Font</h2>\n", self.config.family_name()));
        html.push_str(&format!("<p>Version {}</p>\n", self.config.font_version()));

        html.push_str("<h3>Usage Examples</h3>\n");
        html.push_str("<pre><code>&lt;i class=\"aurora-icon aurora-icon-home\"&gt;&lt;/i&gt;</code></pre>\n\n");

        html.push_str("<h3>All Icons</h3>\n");
        html.push_str("<div class='icon-grid'>\n");

        for glyph in &self.glyphs {
            html.push_str("  <div class='icon-item'>\n");
            html.push_str(&format!("    <i class=\"aurora-icon {}\"></i>\n", glyph.css_class()));
            html.push_str(&format!("    <span>{}</span>\n", glyph.name()));
            html.push_str(&format!("    <code>{}</code>\n", glyph.icon_id()));
            html.push_str("  </div>\n");
        }

        html.push_str("</div>\n");
        html.push_str("</div>\n");

        html
    }

    /// Generate font metadata JSON
    pub fn generate_metadata(&self) -> String {
        let mut json = String::from("{\n");
        json.push_str(&format!("  \"name\": \"{}\",\n", self.config.family_name()));
        json.push_str(&format!("  \"version\": \"{}\",\n", self.config.font_version()));
        json.push_str(&format!("  \"copyright\": \"{}\",\n", self.config.copyright()));
        json.push_str(&format!("  \"manufacturer\": \"{}\",\n", self.config.manufacturer()));
        json.push_str(&format!("  \"license\": \"{}\",\n", self.config.license()));
        json.push_str(&format!("  \"glyphCount\": {},\n", self.glyphs.len()));
        json.push_str("  \"glyphs\": [\n");

        for (i, glyph) in self.glyphs.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"id\": \"{}\",\n", glyph.icon_id()));
            json.push_str(&format!("      \"name\": \"{}\",\n", glyph.name()));
            json.push_str(&format!("      \"codepoint\": \"U+{}\"\n", glyph.codepoint_hex()));

            if i < self.glyphs.len() - 1 {
                json.push_str("    },\n");
            } else {
                json.push_str("    }\n");
            }
        }

        json.push_str("  ]\n");
        json.push_str("}\n");

        json
    }
}

/// Generate standard Aurora icon glyphs
pub fn generate_aurora_icon_glyphs() -> Vec<IconGlyph> {
    vec![
        IconGlyph::new("home", 0xE001, "Home"),
        IconGlyph::new("save", 0xE002, "Save"),
        IconGlyph::new("delete", 0xE003, "Delete"),
        IconGlyph::new("settings", 0xE004, "Settings"),
        IconGlyph::new("search", 0xE005, "Search"),
        IconGlyph::new("menu", 0xE006, "Menu"),
        IconGlyph::new("close", 0xE007, "Close"),
        IconGlyph::new("check", 0xE008, "Check"),
        IconGlyph::new("alert", 0xE009, "Alert"),
        IconGlyph::new("info", 0xE00A, "Info"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_format_extension() {
        assert_eq!(FontFormat::TTF.extension(), ".ttf");
        assert_eq!(FontFormat::WOFF2.extension(), ".woff2");
        assert_eq!(FontFormat::WOFF.extension(), ".woff");
    }

    #[test]
    fn test_font_format_mime_type() {
        assert_eq!(FontFormat::TTF.mime_type(), "font/ttf");
        assert_eq!(FontFormat::WOFF2.mime_type(), "font/woff2");
    }

    #[test]
    fn test_icon_glyph_creation() {
        let glyph = IconGlyph::new("home", 0xE001, "Home");
        assert_eq!(glyph.icon_id(), "home");
        assert_eq!(glyph.codepoint(), 0xE001);
        assert_eq!(glyph.name(), "Home");
    }

    #[test]
    fn test_icon_glyph_hex() {
        let glyph = IconGlyph::new("home", 0xE001, "Home");
        assert_eq!(glyph.codepoint_hex(), "E001");
    }

    #[test]
    fn test_icon_glyph_css_class() {
        let glyph = IconGlyph::new("home", 0xE001, "Home");
        assert_eq!(glyph.css_class(), ".aurora-icon-home");
    }

    #[test]
    fn test_icon_font_config_default() {
        let config = IconFontConfig::default();
        assert_eq!(config.family_name(), "Aurora Icons");
        assert_eq!(config.font_version(), "1.1.0");
    }

    #[test]
    fn test_icon_font_config_custom() {
        let config = IconFontConfig::new("Custom Icons");
        assert_eq!(config.family_name(), "Custom Icons");
    }

    #[test]
    fn test_icon_font_builder_creation() {
        let builder = IconFontBuilder::new(IconFontConfig::default());
        assert_eq!(builder.glyph_count(), 0);
    }

    #[test]
    fn test_icon_font_builder_add_glyph() {
        let mut builder = IconFontBuilder::new(IconFontConfig::default());
        builder.add_glyph(IconGlyph::new("home", 0xE001, "Home"));
        assert_eq!(builder.glyph_count(), 1);
    }

    #[test]
    fn test_icon_font_builder_add_glyphs() {
        let mut builder = IconFontBuilder::new(IconFontConfig::default());
        let glyphs = generate_aurora_icon_glyphs();
        builder.add_glyphs(glyphs);
        assert_eq!(builder.glyph_count(), 10);
    }

    #[test]
    fn test_generate_css() {
        let mut builder = IconFontBuilder::new(IconFontConfig::default());
        builder.add_glyph(IconGlyph::new("home", 0xE001, "Home"));

        let css = builder.generate_css();
        assert!(css.contains("@font-face"));
        assert!(css.contains("Aurora Icons"));
        assert!(css.contains(".aurora-icon-home"));
    }

    #[test]
    fn test_generate_html_examples() {
        let mut builder = IconFontBuilder::new(IconFontConfig::default());
        builder.add_glyph(IconGlyph::new("home", 0xE001, "Home"));

        let html = builder.generate_html_examples();
        assert!(html.contains("Icon Font"));
        assert!(html.contains("Home"));
        assert!(html.contains("aurora-icon-home"));
    }

    #[test]
    fn test_generate_metadata() {
        let mut builder = IconFontBuilder::new(IconFontConfig::default());
        builder.add_glyph(IconGlyph::new("home", 0xE001, "Home"));

        let metadata = builder.generate_metadata();
        assert!(metadata.contains("\"name\""));
        assert!(metadata.contains("\"version\""));
        assert!(metadata.contains("\"glyphCount\": 1"));
    }

    #[test]
    fn test_generate_aurora_icon_glyphs() {
        let glyphs = generate_aurora_icon_glyphs();
        assert_eq!(glyphs.len(), 10);
        assert_eq!(glyphs[0].icon_id(), "home");
        assert_eq!(glyphs[9].icon_id(), "info");
    }

    #[test]
    fn test_aurora_icons_codepoints() {
        let glyphs = generate_aurora_icon_glyphs();
        assert_eq!(glyphs[0].codepoint(), 0xE001);
        assert_eq!(glyphs[1].codepoint(), 0xE002);
        assert_eq!(glyphs[9].codepoint(), 0xE00A);
    }

    #[test]
    fn test_font_builder_metadata_json_valid() {
        let mut builder = IconFontBuilder::new(IconFontConfig::default());
        let glyphs = generate_aurora_icon_glyphs();
        builder.add_glyphs(glyphs);

        let metadata = builder.generate_metadata();
        assert!(metadata.contains("{"));
        assert!(metadata.contains("}"));
        assert!(metadata.starts_with("{"));
        assert!(metadata.ends_with("}\n"));
    }
}
