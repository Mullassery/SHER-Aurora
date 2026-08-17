use serde::{Deserialize, Serialize};

/// Spacing scale — 8px baseline grid
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpacingScale {
    pub xxs: u16,  // 2px — micro adjustments
    pub xs: u16,   // 4px — tight spacing
    pub sm: u16,   // 8px — grid unit
    pub md: u16,   // 12px — standard padding
    pub lg: u16,   // 16px — section spacing
    pub xl: u16,   // 24px — large spacing
    pub xxl: u16,  // 32px — layout spacing
    pub xxxl: u16, // 48px — screen margins
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xxs: 2,
            xs: 4,
            sm: 8,
            md: 12,
            lg: 16,
            xl: 24,
            xxl: 32,
            xxxl: 48,
        }
    }
}

/// Individual spacing value
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spacing(pub u16);

impl Spacing {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn px(&self) -> String {
        format!("{}px", self.0)
    }

    pub fn rem(&self, base: u16) -> f32 {
        self.0 as f32 / base as f32
    }
}

impl From<u16> for Spacing {
    fn from(value: u16) -> Self {
        Spacing(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_spacing_scale() {
        let scale = SpacingScale::default();
        assert_eq!(scale.xxs, 2);
        assert_eq!(scale.sm, 8);
        assert_eq!(scale.xxxl, 48);
    }

    #[test]
    fn test_spacing_px_format() {
        let spacing = Spacing(16);
        assert_eq!(spacing.px(), "16px");
    }

    #[test]
    fn test_spacing_rem_conversion() {
        let spacing = Spacing(16);
        assert_eq!(spacing.rem(16), 1.0);
        assert_eq!(spacing.rem(8), 2.0);
    }
}
