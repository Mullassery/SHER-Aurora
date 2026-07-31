use serde::{Deserialize, Serialize};

/// Aurora theme for GTK
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    OLED,
    HDR,
}

impl Theme {
    pub fn name(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::OLED => "OLED",
            Theme::HDR => "HDR",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "aurora-light",
            Theme::Dark => "aurora-dark",
            Theme::OLED => "aurora-oled",
            Theme::HDR => "aurora-hdr",
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}
