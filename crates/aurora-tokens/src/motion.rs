use serde::{Deserialize, Serialize};

/// Animation duration in milliseconds
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum AnimationDuration {
    Instant = 80,    // Micro-interactions, tooltips
    Fast = 120,      // Quick feedback, hover states
    Normal = 220,    // Standard transitions, state changes
    Slow = 350,      // Complex animations, entrance animations
    Dramatic = 500,  // Page transitions, full-screen changes
}

impl AnimationDuration {
    pub fn ms(&self) -> u16 {
        *self as u16
    }

    pub fn css(&self) -> String {
        format!("{}ms", self.ms())
    }
}

/// Spring physics configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpringConfig {
    pub mass: f32,      // Typically 1.0
    pub tension: f32,   // Stiffness (280 is standard)
    pub friction: f32,  // Damping (60 is standard)
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            mass: 1.0,
            tension: 280.0,
            friction: 60.0,
        }
    }
}

impl SpringConfig {
    pub fn new(mass: f32, tension: f32, friction: f32) -> Self {
        Self {
            mass,
            tension,
            friction,
        }
    }

    pub fn gentle() -> Self {
        Self {
            mass: 1.0,
            tension: 170.0,
            friction: 26.0,
        }
    }

    pub fn snappy() -> Self {
        Self {
            mass: 1.0,
            tension: 350.0,
            friction: 40.0,
        }
    }
}

/// Easing function definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EasingFunction {
    /// Cubic bezier curve
    CubicBezier(f32, f32, f32, f32),
    /// Linear
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in-out
    EaseInOut,
}

impl EasingFunction {
    pub fn to_css(&self) -> String {
        match self {
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            EasingFunction::Linear => "linear".to_string(),
            EasingFunction::EaseIn => "ease-in".to_string(),
            EasingFunction::EaseOut => "ease-out".to_string(),
            EasingFunction::EaseInOut => "ease-in-out".to_string(),
        }
    }

    pub fn bounce_in() -> Self {
        EasingFunction::CubicBezier(0.34, 1.56, 0.64, 1.0)
    }

    pub fn material_ease_out() -> Self {
        EasingFunction::CubicBezier(0.4, 0.0, 0.2, 1.0)
    }

    pub fn material_ease_in_out() -> Self {
        EasingFunction::CubicBezier(0.4, 0.0, 0.2, 1.0)
    }
}

/// Motion design scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionScale {
    pub instant: AnimationDuration,
    pub fast: AnimationDuration,
    pub normal: AnimationDuration,
    pub slow: AnimationDuration,
    pub dramatic: AnimationDuration,
    pub spring_config: SpringConfig,
}

impl Default for MotionScale {
    fn default() -> Self {
        Self {
            instant: AnimationDuration::Instant,
            fast: AnimationDuration::Fast,
            normal: AnimationDuration::Normal,
            slow: AnimationDuration::Slow,
            dramatic: AnimationDuration::Dramatic,
            spring_config: SpringConfig::default(),
        }
    }
}

impl MotionScale {
    pub fn get_duration(&self, duration_type: AnimationDuration) -> u16 {
        duration_type.ms()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_duration_ms() {
        assert_eq!(AnimationDuration::Instant.ms(), 80);
        assert_eq!(AnimationDuration::Normal.ms(), 220);
        assert_eq!(AnimationDuration::Dramatic.ms(), 500);
    }

    #[test]
    fn test_animation_duration_css() {
        assert_eq!(AnimationDuration::Fast.css(), "120ms");
    }

    #[test]
    fn test_spring_config_defaults() {
        let spring = SpringConfig::default();
        assert_eq!(spring.mass, 1.0);
        assert_eq!(spring.tension, 280.0);
    }

    #[test]
    fn test_spring_config_presets() {
        let gentle = SpringConfig::gentle();
        let snappy = SpringConfig::snappy();
        assert!(snappy.tension > gentle.tension);
    }

    #[test]
    fn test_easing_css_format() {
        let easing = EasingFunction::CubicBezier(0.4, 0.0, 0.2, 1.0);
        assert_eq!(easing.to_css(), "cubic-bezier(0.4, 0, 0.2, 1)");
    }

    #[test]
    fn test_easing_presets() {
        let bounce = EasingFunction::bounce_in();
        let material = EasingFunction::material_ease_out();
        assert!(bounce.to_css().contains("1.56"));
        assert!(material.to_css().contains("0.4"));
    }

    #[test]
    fn test_motion_scale_default() {
        let motion = MotionScale::default();
        assert_eq!(motion.instant.ms(), 80);
        assert_eq!(motion.dramatic.ms(), 500);
    }
}
