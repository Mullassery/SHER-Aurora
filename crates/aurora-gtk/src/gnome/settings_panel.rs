//! GNOME Settings integration panel

use aurora_color::ThemeName;

/// Aurora settings panel for GNOME Settings
pub struct SettingsPanel {
    title: String,
    sections: Vec<SettingsSection>,
}

/// Individual settings section
#[derive(Debug, Clone)]
pub struct SettingsSection {
    title: String,
    settings: Vec<Setting>,
}

/// Individual setting item
#[derive(Debug, Clone)]
pub struct Setting {
    key: String,
    label: String,
    value_type: SettingType,
    current_value: String,
}

/// Setting value type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingType {
    String,
    Boolean,
    Integer,
    Double,
    Enum,
}

impl SettingsPanel {
    /// Create new settings panel
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            sections: vec![
                Self::theme_section(),
                Self::sound_section(),
                Self::accessibility_section(),
            ],
        }
    }

    /// Get panel title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get all sections
    pub fn sections(&self) -> &[SettingsSection] {
        &self.sections
    }

    /// Get section by title
    pub fn section(&self, title: &str) -> Option<&SettingsSection> {
        self.sections.iter().find(|s| s.title == title)
    }

    /// Theme section
    fn theme_section() -> SettingsSection {
        SettingsSection {
            title: "Appearance".to_string(),
            settings: vec![
                Setting {
                    key: "theme".to_string(),
                    label: "Color Scheme".to_string(),
                    value_type: SettingType::Enum,
                    current_value: "light".to_string(),
                },
                Setting {
                    key: "high-contrast".to_string(),
                    label: "High Contrast".to_string(),
                    value_type: SettingType::Boolean,
                    current_value: "false".to_string(),
                },
                Setting {
                    key: "text-scale".to_string(),
                    label: "Text Scaling".to_string(),
                    value_type: SettingType::Double,
                    current_value: "1.0".to_string(),
                },
            ],
        }
    }

    /// Sound section
    fn sound_section() -> SettingsSection {
        SettingsSection {
            title: "Sound".to_string(),
            settings: vec![
                Setting {
                    key: "sound-enabled".to_string(),
                    label: "Sound Feedback".to_string(),
                    value_type: SettingType::Boolean,
                    current_value: "true".to_string(),
                },
                Setting {
                    key: "sound-volume".to_string(),
                    label: "Volume".to_string(),
                    value_type: SettingType::Double,
                    current_value: "0.8".to_string(),
                },
                Setting {
                    key: "sound-theme".to_string(),
                    label: "Sound Theme".to_string(),
                    value_type: SettingType::Enum,
                    current_value: "standard".to_string(),
                },
            ],
        }
    }

    /// Accessibility section
    fn accessibility_section() -> SettingsSection {
        SettingsSection {
            title: "Accessibility".to_string(),
            settings: vec![Setting {
                key: "reduce-motion".to_string(),
                label: "Reduce Motion".to_string(),
                value_type: SettingType::Boolean,
                current_value: "false".to_string(),
            }],
        }
    }

    /// Generate HTML representation
    pub fn to_html(&self) -> String {
        let mut html = format!(
            r#"<div class="aurora-settings-panel">
  <h1>{}</h1>"#,
            self.title
        );

        for section in &self.sections {
            html.push_str(&format!(
                r#"
  <div class="settings-section">
    <h2>{}</h2>
    <ul>"#,
                section.title
            ));

            for setting in &section.settings {
                html.push_str(&format!(
                    r#"
      <li class="setting-item">
        <label for="{}">{}</label>
        <input id="{}" type="text" value="{}" />
      </li>"#,
                    setting.key, setting.label, setting.key, setting.current_value
                ));
            }

            html.push_str("\n    </ul>\n  </div>");
        }

        html.push_str("\n</div>");
        html
    }

    /// Get setting count
    pub fn setting_count(&self) -> usize {
        self.sections.iter().map(|s| s.settings.len()).sum()
    }

    /// Get section count
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new("Aurora Settings")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_panel_creation() {
        let panel = SettingsPanel::new("Test Panel");
        assert_eq!(panel.title(), "Test Panel");
    }

    #[test]
    fn test_section_count() {
        let panel = SettingsPanel::new("Panel");
        assert_eq!(panel.section_count(), 3);
    }

    #[test]
    fn test_setting_count() {
        let panel = SettingsPanel::new("Panel");
        assert_eq!(panel.setting_count(), 7); // 3 + 3 + 1 settings
    }

    #[test]
    fn test_get_section() {
        let panel = SettingsPanel::new("Panel");
        let section = panel.section("Appearance");
        assert!(section.is_some());
        assert_eq!(section.unwrap().title, "Appearance");
    }

    #[test]
    fn test_theme_section() {
        let section = SettingsPanel::theme_section();
        assert_eq!(section.title, "Appearance");
        assert_eq!(section.settings.len(), 3);
    }

    #[test]
    fn test_sound_section() {
        let section = SettingsPanel::sound_section();
        assert_eq!(section.title, "Sound");
        assert_eq!(section.settings.len(), 3);
    }

    #[test]
    fn test_accessibility_section() {
        let section = SettingsPanel::accessibility_section();
        assert_eq!(section.title, "Accessibility");
        assert_eq!(section.settings.len(), 1);
    }

    #[test]
    fn test_html_generation() {
        let panel = SettingsPanel::new("Test");
        let html = panel.to_html();
        assert!(html.contains("aurora-settings-panel"));
        assert!(html.contains("Appearance"));
        assert!(html.contains("Sound"));
        assert!(html.contains("Accessibility"));
    }

    #[test]
    fn test_setting_type_equality() {
        assert_eq!(SettingType::String, SettingType::String);
        assert_ne!(SettingType::String, SettingType::Boolean);
    }

    #[test]
    fn test_default() {
        let panel = SettingsPanel::default();
        assert_eq!(panel.title(), "Aurora Settings");
    }
}
