use serde::{Deserialize, Serialize};

/// Border radius scale
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RadiusScale {
    pub xs: u16, // 4px — subtle softness
    pub sm: u16, // 8px — buttons, cards, inputs
    pub md: u16, // 12px — larger containers, modals
    pub lg: u16, // 16px — prominent surfaces
    pub xl: u16, // 24px — large cards, full-width containers
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            xs: 4,
            sm: 8,
            md: 12,
            lg: 16,
            xl: 24,
        }
    }
}

/// Individual radius value
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Radius(pub u16);

impl Radius {
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

impl From<u16> for Radius {
    fn from(value: u16) -> Self {
        Radius(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_radius_scale() {
        let scale = RadiusScale::default();
        assert_eq!(scale.xs, 4);
        assert_eq!(scale.md, 12);
        assert_eq!(scale.xl, 24);
    }

    #[test]
    fn test_radius_px_format() {
        let radius = Radius(8);
        assert_eq!(radius.px(), "8px");
    }

    #[test]
    fn test_radius_rem_conversion() {
        let radius = Radius(16);
        assert_eq!(radius.rem(16), 1.0);
    }
}
