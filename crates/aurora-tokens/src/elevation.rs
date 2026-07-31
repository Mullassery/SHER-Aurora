use serde::{Deserialize, Serialize};

/// Elevation level — semantic shadow definitions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ElevationLevel {
    Level1,  // Subtle hover states, tooltips
    Level2,  // Popovers, dropdowns, cards
    Level3,  // Modals, floating windows
    Level4,  // Prominent surfaces, app windows
    Level5,  // Full-screen overlays, system alerts
}

/// Shadow definition (CSS box-shadow compatible)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shadow {
    pub offset_x: u16,
    pub offset_y: u16,
    pub blur: u16,
    pub spread: u16,
    pub color: String,  // rgba format
}

impl Shadow {
    pub fn to_css(&self) -> String {
        format!(
            "{}px {}px {}px {}px {}",
            self.offset_x, self.offset_y, self.blur, self.spread, self.color
        )
    }
}

/// Elevation system with theme-aware shadows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elevation {
    pub level1: Shadow,
    pub level2: Shadow,
    pub level3: Shadow,
    pub level4: Shadow,
    pub level5: Shadow,
}

impl Default for Elevation {
    fn default() -> Self {
        Self::light()
    }
}

impl Elevation {
    pub fn light() -> Self {
        Self {
            level1: Shadow {
                offset_x: 0,
                offset_y: 1,
                blur: 2,
                spread: 0,
                color: "rgba(0, 0, 0, 0.08)".to_string(),
            },
            level2: Shadow {
                offset_x: 0,
                offset_y: 3,
                blur: 8,
                spread: 0,
                color: "rgba(0, 0, 0, 0.12)".to_string(),
            },
            level3: Shadow {
                offset_x: 0,
                offset_y: 8,
                blur: 16,
                spread: 0,
                color: "rgba(0, 0, 0, 0.16)".to_string(),
            },
            level4: Shadow {
                offset_x: 0,
                offset_y: 12,
                blur: 24,
                spread: 0,
                color: "rgba(0, 0, 0, 0.20)".to_string(),
            },
            level5: Shadow {
                offset_x: 0,
                offset_y: 16,
                blur: 32,
                spread: 0,
                color: "rgba(0, 0, 0, 0.24)".to_string(),
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            level1: Shadow {
                offset_x: 0,
                offset_y: 1,
                blur: 2,
                spread: 0,
                color: "rgba(255, 255, 255, 0.04)".to_string(),
            },
            level2: Shadow {
                offset_x: 0,
                offset_y: 3,
                blur: 8,
                spread: 0,
                color: "rgba(255, 255, 255, 0.06)".to_string(),
            },
            level3: Shadow {
                offset_x: 0,
                offset_y: 8,
                blur: 16,
                spread: 0,
                color: "rgba(255, 255, 255, 0.08)".to_string(),
            },
            level4: Shadow {
                offset_x: 0,
                offset_y: 12,
                blur: 24,
                spread: 0,
                color: "rgba(255, 255, 255, 0.10)".to_string(),
            },
            level5: Shadow {
                offset_x: 0,
                offset_y: 16,
                blur: 32,
                spread: 0,
                color: "rgba(255, 255, 255, 0.12)".to_string(),
            },
        }
    }

    pub fn get_shadow(&self, level: ElevationLevel) -> &Shadow {
        match level {
            ElevationLevel::Level1 => &self.level1,
            ElevationLevel::Level2 => &self.level2,
            ElevationLevel::Level3 => &self.level3,
            ElevationLevel::Level4 => &self.level4,
            ElevationLevel::Level5 => &self.level5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_elevation() {
        let elevation = Elevation::light();
        assert!(elevation.level1.color.contains("rgba(0, 0, 0, 0.08)"));
        assert!(elevation.level5.color.contains("rgba(0, 0, 0, 0.24)"));
    }

    #[test]
    fn test_dark_elevation() {
        let elevation = Elevation::dark();
        assert!(elevation.level1.color.contains("rgba(255, 255, 255, 0.04)"));
    }

    #[test]
    fn test_shadow_css_format() {
        let shadow = Shadow {
            offset_x: 0,
            offset_y: 1,
            blur: 2,
            spread: 0,
            color: "rgba(0, 0, 0, 0.08)".to_string(),
        };
        assert_eq!(shadow.to_css(), "0px 1px 2px 0px rgba(0, 0, 0, 0.08)");
    }

    #[test]
    fn test_get_shadow() {
        let elevation = Elevation::light();
        let level2 = elevation.get_shadow(ElevationLevel::Level2);
        assert_eq!(level2.blur, 8);
    }
}
