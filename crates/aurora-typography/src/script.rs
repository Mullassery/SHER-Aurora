use serde::{Deserialize, Serialize};

/// Writing script family
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Script {
    /// Latin scripts (English, French, German, etc.)
    Latin,
    /// Chinese, Japanese (Kanji), Korean (Hanja)
    CJK,
    /// Arabic, Hebrew
    RTL,
    /// Devanagari (Hindi, Sanskrit)
    Devanagari,
    /// Thai
    Thai,
}

impl Script {
    pub fn name(&self) -> &'static str {
        match self {
            Script::Latin => "Latin",
            Script::CJK => "CJK",
            Script::RTL => "RTL",
            Script::Devanagari => "Devanagari",
            Script::Thai => "Thai",
        }
    }

    /// Get typographic adjustments for this script
    pub fn adjustment(&self) -> ScriptAdjustment {
        match self {
            Script::Latin => ScriptAdjustment {
                line_height_multiplier: 1.0,
                optimal_line_length: 70,
                prefers_serifs: false,
            },
            Script::CJK => ScriptAdjustment {
                line_height_multiplier: 1.1,
                optimal_line_length: 50,
                prefers_serifs: false,
            },
            Script::RTL => ScriptAdjustment {
                line_height_multiplier: 1.0,
                optimal_line_length: 65,
                prefers_serifs: false,
            },
            Script::Devanagari => ScriptAdjustment {
                line_height_multiplier: 1.15,
                optimal_line_length: 60,
                prefers_serifs: false,
            },
            Script::Thai => ScriptAdjustment {
                line_height_multiplier: 1.2,
                optimal_line_length: 55,
                prefers_serifs: false,
            },
        }
    }

    /// Get optimal line length in characters for readability
    pub fn optimal_line_length(&self) -> u16 {
        self.adjustment().optimal_line_length
    }

    /// Detect script from text sample (simple heuristic)
    pub fn detect(text: &str) -> Self {
        // CJK detection (CJK Unicode ranges)
        if text.chars().any(|c| {
            matches!(c as u32,
                0x4E00..=0x9FFF |   // CJK Unified Ideographs
                0x3040..=0x309F |   // Hiragana
                0x30A0..=0x30FF |   // Katakana
                0xAC00..=0xD7AF     // Hangul
            )
        }) {
            return Script::CJK;
        }

        // RTL detection (Arabic, Hebrew)
        if text.chars().any(|c| {
            matches!(c as u32,
                0x0590..=0x06FF     // Hebrew and Arabic (contiguous range)
            )
        }) {
            return Script::RTL;
        }

        // Devanagari
        if text.chars().any(|c| {
            matches!(c as u32,
                0x0900..=0x097F     // Devanagari
            )
        }) {
            return Script::Devanagari;
        }

        // Thai
        if text.chars().any(|c| {
            matches!(c as u32,
                0x0E00..=0x0E7F     // Thai
            )
        }) {
            return Script::Thai;
        }

        // Default to Latin
        Script::Latin
    }
}

/// Typographic adjustments per script
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScriptAdjustment {
    /// Line height multiplier (CJK benefits from extra space)
    pub line_height_multiplier: f32,
    /// Optimal line length in characters
    pub optimal_line_length: u16,
    /// Whether this script prefers serifs
    pub prefers_serifs: bool,
}

impl ScriptAdjustment {
    pub fn new(line_height_multiplier: f32, optimal_line_length: u16, prefers_serifs: bool) -> Self {
        Self {
            line_height_multiplier,
            optimal_line_length,
            prefers_serifs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_names() {
        assert_eq!(Script::Latin.name(), "Latin");
        assert_eq!(Script::CJK.name(), "CJK");
    }

    #[test]
    fn test_script_adjustment_latin() {
        let adj = Script::Latin.adjustment();
        assert_eq!(adj.line_height_multiplier, 1.0);
        assert!(adj.optimal_line_length > 60);
    }

    #[test]
    fn test_script_adjustment_cjk() {
        let adj = Script::CJK.adjustment();
        assert!(adj.line_height_multiplier > 1.0);
        assert!(adj.optimal_line_length < 70);
    }

    #[test]
    fn test_script_detection_latin() {
        let script = Script::detect("Hello, world!");
        assert_eq!(script, Script::Latin);
    }

    #[test]
    fn test_script_detection_cjk() {
        let script = Script::detect("你好世界");
        assert_eq!(script, Script::CJK);
    }

    #[test]
    fn test_script_detection_rtl() {
        let script = Script::detect("السلام عليكم");
        assert_eq!(script, Script::RTL);
    }

    #[test]
    fn test_script_optimal_line_length() {
        let latin_length = Script::Latin.optimal_line_length();
        let cjk_length = Script::CJK.optimal_line_length();
        assert!(latin_length > cjk_length);
    }

    #[test]
    fn test_script_adjustment_devanagari() {
        let adj = Script::Devanagari.adjustment();
        assert!(adj.line_height_multiplier >= 1.1);
    }
}
