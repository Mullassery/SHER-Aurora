//! dconf schema for Aurora GNOME integration

/// Aurora dconf schema builder
pub struct DConfSchema;

impl DConfSchema {
    /// Get the dconf schema XML for Aurora settings
    pub fn schema_xml() -> &'static str {
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<gschema2>\n\
  <schema id=\"org.gnome.desktop.interface.aurora\" path=\"/org/gnome/desktop/interface/aurora/\"\n\
          gettext-domain=\"aurora\">\n\
\n\
    <!-- Theme Settings -->\n\
    <key type=\"s\" name=\"theme\">\n\
      <default>'light'</default>\n\
      <summary>Aurora theme variant</summary>\n\
      <description>The Aurora theme to use: light, dark, oled, or hdr</description>\n\
    </key>\n\
\n\
    <key type=\"b\" name=\"high-contrast\">\n\
      <default>false</default>\n\
      <summary>Enable high contrast mode</summary>\n\
      <description>Use high contrast colors for improved visibility</description>\n\
    </key>\n\
\n\
    <key type=\"b\" name=\"reduce-motion\">\n\
      <default>false</default>\n\
      <summary>Reduce motion</summary>\n\
      <description>Reduce or disable animations for accessibility</description>\n\
    </key>\n\
\n\
    <key type=\"d\" name=\"text-scale\">\n\
      <default>1.0</default>\n\
      <summary>Text scaling factor</summary>\n\
      <description>Scale text by this factor (0.5-2.0)</description>\n\
    </key>\n\
\n\
    <!-- Sound Settings -->\n\
    <key type=\"b\" name=\"sound-enabled\">\n\
      <default>true</default>\n\
      <summary>Enable Aurora sounds</summary>\n\
      <description>Enable or disable Aurora notification sounds</description>\n\
    </key>\n\
\n\
    <key type=\"d\" name=\"sound-volume\">\n\
      <default>0.8</default>\n\
      <summary>Aurora sound volume</summary>\n\
      <description>Volume for Aurora notification sounds (0.0-1.0)</description>\n\
    </key>\n\
\n\
    <key type=\"s\" name=\"sound-theme\">\n\
      <default>'standard'</default>\n\
      <summary>Aurora sound theme</summary>\n\
      <description>The sound theme to use: standard or subtle</description>\n\
    </key>\n\
\n\
    <!-- Color Settings -->\n\
    <key type=\"s\" name=\"primary-color\">\n\
      <default>'#003D99'</default>\n\
      <summary>Primary color override</summary>\n\
      <description>Override the primary theme color (hex format)</description>\n\
    </key>\n\
\n\
    <key type=\"s\" name=\"accent-color\">\n\
      <default>'#AA0044'</default>\n\
      <summary>Accent color override</summary>\n\
      <description>Override the accent theme color (hex format)</description>\n\
    </key>\n\
\n\
  </schema>\n\
</gschema2>"
    }

    /// Get schema ID
    pub fn schema_id() -> &'static str {
        "org.gnome.desktop.interface.aurora"
    }

    /// Get schema path
    pub fn schema_path() -> &'static str {
        "/org/gnome/desktop/interface/aurora/"
    }

    /// Generate schema installation command
    pub fn install_command() -> String {
        format!(
            "glib-compile-schemas {}/share/glib-2.0/schemas/",
            std::env::var("PREFIX").unwrap_or_else(|_| "/usr/local".to_string())
        )
    }

    /// Get schema as formatted text
    pub fn to_string() -> String {
        Self::schema_xml().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_id() {
        assert_eq!(
            DConfSchema::schema_id(),
            "org.gnome.desktop.interface.aurora"
        );
    }

    #[test]
    fn test_schema_path() {
        assert_eq!(
            DConfSchema::schema_path(),
            "/org/gnome/desktop/interface/aurora/"
        );
    }

    #[test]
    fn test_schema_xml_valid() {
        let xml = DConfSchema::schema_xml();
        assert!(xml.contains("<?xml version"));
        assert!(xml.contains("gschema2"));
        assert!(xml.contains("aurora"));
    }

    #[test]
    fn test_schema_contains_theme_key() {
        let xml = DConfSchema::schema_xml();
        assert!(xml.contains("name=\"theme\""));
        assert!(xml.contains("light"));
    }

    #[test]
    fn test_schema_contains_sound_key() {
        let xml = DConfSchema::schema_xml();
        assert!(xml.contains("name=\"sound-enabled\""));
        assert!(xml.contains("name=\"sound-volume\""));
    }

    #[test]
    fn test_schema_contains_accessibility_keys() {
        let xml = DConfSchema::schema_xml();
        assert!(xml.contains("name=\"high-contrast\""));
        assert!(xml.contains("name=\"reduce-motion\""));
    }

    #[test]
    fn test_install_command_generation() {
        let cmd = DConfSchema::install_command();
        assert!(cmd.contains("glib-compile-schemas"));
    }
}
