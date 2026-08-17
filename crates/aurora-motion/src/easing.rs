use serde::{Deserialize, Serialize};

/// Easing function for non-spring animations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EasingFunction {
    /// Linear: no easing, constant speed
    Linear,
    /// Ease in: slow start, fast end
    EaseIn,
    /// Ease out: fast start, slow end (common)
    EaseOut,
    /// Ease in-out: slow start and end, fast middle
    EaseInOut,
    /// Cubic bezier: custom easing curve
    CubicBezier(f32, f32, f32, f32),
}

impl EasingFunction {
    /// Standard Material Design ease-out curve
    pub fn material_ease_out() -> Self {
        Self::CubicBezier(0.4, 0.0, 0.2, 1.0)
    }

    /// Standard Material Design ease-in-out curve
    pub fn material_ease_in_out() -> Self {
        Self::CubicBezier(0.4, 0.0, 0.2, 1.0)
    }

    /// Bounce-in (for entrance animations)
    pub fn bounce_in() -> Self {
        Self::CubicBezier(0.34, 1.56, 0.64, 1.0)
    }

    /// Quick bounce-out
    pub fn bounce_out() -> Self {
        Self::CubicBezier(0.16, 1.0, 0.3, 1.0)
    }

    /// Evaluate easing at progress (0.0–1.0)
    pub fn evaluate(&self, progress: f32) -> f32 {
        let t = progress.clamp(0.0, 1.0);

        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => t * (2.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingFunction::CubicBezier(x1, y1, x2, y2) => cubic_bezier(t, *x1, *y1, *x2, *y2),
        }
    }

    /// To CSS string
    pub fn to_css(&self) -> String {
        match self {
            EasingFunction::Linear => "linear".to_string(),
            EasingFunction::EaseIn => "ease-in".to_string(),
            EasingFunction::EaseOut => "ease-out".to_string(),
            EasingFunction::EaseInOut => "ease-in-out".to_string(),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
        }
    }
}

/// Cubic Bézier curve evaluation (simplified)
/// Uses approximation for performance
fn cubic_bezier(t: f32, _x1: f32, y1: f32, _x2: f32, y2: f32) -> f32 {
    // Simplified cubic-bezier approximation (ignores x components for speed)
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;
    let t2 = t * t;
    let t3 = t2 * t;

    // Bézier formula: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
    let y = mt3 * 0.0 + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * 1.0;

    y.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(0.5), 0.5);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let easing = EasingFunction::EaseIn;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert!(easing.evaluate(0.5) < 0.5); // Slower at start
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out() {
        let easing = EasingFunction::EaseOut;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert!(easing.evaluate(0.5) > 0.5); // Faster at start
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_cubic_bezier() {
        let easing = EasingFunction::CubicBezier(0.4, 0.0, 0.2, 1.0);
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert!(easing.evaluate(0.5) >= 0.0);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_to_css() {
        assert_eq!(EasingFunction::Linear.to_css(), "linear");
        assert_eq!(EasingFunction::EaseOut.to_css(), "ease-out");
        assert!(EasingFunction::CubicBezier(0.4, 0.0, 0.2, 1.0)
            .to_css()
            .contains("cubic-bezier"));
    }

    #[test]
    fn test_material_ease_out() {
        let easing = EasingFunction::material_ease_out();
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_bounce_in() {
        let easing = EasingFunction::bounce_in();
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_clamping() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.evaluate(-0.5), 0.0); // Clamped to 0
        assert_eq!(easing.evaluate(1.5), 1.0); // Clamped to 1
    }
}
