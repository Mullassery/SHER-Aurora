use crate::{Animation, EasingFunction, SpringConfig};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Window motion event
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WindowEvent {
    Open,
    Close,
    Minimize,
    Maximize,
    Restore,
    Focus,
    Blur,
}

/// Window animation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowAnimationConfig {
    /// Open animation: origin → expansion → settle
    pub open_duration: Duration,
    pub open_spring: SpringConfig,

    /// Close animation: contract → fade → collapse
    pub close_duration: Duration,
    pub close_easing: EasingFunction,

    /// Minimize/maximize animations
    pub minimize_duration: Duration,
    pub maximize_duration: Duration,
}

impl Default for WindowAnimationConfig {
    fn default() -> Self {
        Self {
            open_duration: Duration::from_millis(350),
            open_spring: SpringConfig::standard(),
            close_duration: Duration::from_millis(200),
            close_easing: EasingFunction::EaseOut,
            minimize_duration: Duration::from_millis(250),
            maximize_duration: Duration::from_millis(300),
        }
    }
}

/// Window motion state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WindowState {
    Hidden,
    Opening,
    Visible,
    Closing,
    Minimized,
    Maximized,
}

/// Window animation sequence
#[derive(Debug, Clone)]
pub struct WindowAnimation {
    pub event: WindowEvent,
    pub config: WindowAnimationConfig,
    pub state: WindowState,

    /// Scale animation (0.0 → 1.0)
    pub scale_anim: Option<Animation>,
    /// Opacity animation (0.0 → 1.0)
    pub opacity_anim: Option<Animation>,
    /// Position animation (if moving)
    pub position_anim: Option<Animation>,
}

impl WindowAnimation {
    /// Create animation for window opening
    pub fn open(config: WindowAnimationConfig) -> crate::errors::MotionResult<Self> {
        // Scale: 0.95 → 1.0 (subtle expansion)
        let scale_anim = Animation::spring("scale", 0.95, 1.0, config.open_spring)?;

        // Opacity: 0.0 → 1.0 (fade in)
        let opacity_anim = Animation::tween(
            "opacity",
            0.0,
            1.0,
            config.open_duration,
            EasingFunction::EaseOut,
        );

        let mut seq = Self {
            event: WindowEvent::Open,
            config,
            state: WindowState::Opening,
            scale_anim: Some(scale_anim),
            opacity_anim: Some(opacity_anim),
            position_anim: None,
        };

        seq.start();
        Ok(seq)
    }

    /// Create animation for window closing
    pub fn close(config: WindowAnimationConfig) -> Self {
        // Scale: 1.0 → 0.95 (subtle contraction)
        let scale_anim = Animation::tween(
            "scale",
            1.0,
            0.95,
            config.close_duration,
            config.close_easing,
        );

        // Opacity: 1.0 → 0.0 (fade out)
        let opacity_anim = Animation::tween(
            "opacity",
            1.0,
            0.0,
            config.close_duration,
            config.close_easing,
        );

        let mut seq = Self {
            event: WindowEvent::Close,
            config,
            state: WindowState::Closing,
            scale_anim: Some(scale_anim),
            opacity_anim: Some(opacity_anim),
            position_anim: None,
        };

        seq.start();
        seq
    }

    /// Create animation for window minimizing
    pub fn minimize(config: WindowAnimationConfig) -> Self {
        // Scale: 1.0 → 0.5 (shrink toward taskbar)
        let scale_anim = Animation::tween(
            "scale",
            1.0,
            0.5,
            config.minimize_duration,
            EasingFunction::EaseInOut,
        );

        // Opacity: 1.0 → 0.0 (fade out)
        let opacity_anim = Animation::tween(
            "opacity",
            1.0,
            0.0,
            config.minimize_duration,
            EasingFunction::EaseOut,
        );

        let mut seq = Self {
            event: WindowEvent::Minimize,
            config,
            state: WindowState::Minimized,
            scale_anim: Some(scale_anim),
            opacity_anim: Some(opacity_anim),
            position_anim: None,
        };

        seq.start();
        seq
    }

    /// Create animation for window maximizing
    pub fn maximize(config: WindowAnimationConfig) -> crate::errors::MotionResult<Self> {
        // Scale: 1.0 → 1.05 (slight expand)
        let scale_anim = Animation::spring("scale", 1.0, 1.05, config.open_spring)?;

        let mut seq = Self {
            event: WindowEvent::Maximize,
            config,
            state: WindowState::Maximized,
            scale_anim: Some(scale_anim),
            opacity_anim: None,
            position_anim: None,
        };

        seq.start();
        Ok(seq)
    }

    /// Start animations
    pub fn start(&mut self) {
        if let Some(ref mut anim) = self.scale_anim {
            anim.start();
        }
        if let Some(ref mut anim) = self.opacity_anim {
            anim.start();
        }
        if let Some(ref mut anim) = self.position_anim {
            anim.start();
        }
    }

    /// Advance animations by delta time
    pub fn advance(&mut self, delta: Duration) {
        if let Some(ref mut anim) = self.scale_anim {
            anim.advance(delta);
        }
        if let Some(ref mut anim) = self.opacity_anim {
            anim.advance(delta);
        }
        if let Some(ref mut anim) = self.position_anim {
            anim.advance(delta);
        }

        // Update state
        if self.event == WindowEvent::Open {
            if self.is_finished() {
                self.state = WindowState::Visible;
            }
        } else if self.event == WindowEvent::Close {
            if self.is_finished() {
                self.state = WindowState::Hidden;
            }
        }
    }

    /// Get current scale (0.0–1.0)
    pub fn scale(&self) -> f32 {
        self.scale_anim
            .as_ref()
            .map(|a| a.current())
            .unwrap_or(1.0)
    }

    /// Get current opacity (0.0–1.0)
    pub fn opacity(&self) -> f32 {
        self.opacity_anim
            .as_ref()
            .map(|a| a.current())
            .unwrap_or(1.0)
    }

    /// Get current position offset (if animating)
    pub fn position_offset(&self) -> (f32, f32) {
        if let Some(anim) = &self.position_anim {
            let progress = anim.progress();
            (progress * 100.0, progress * 100.0) // Example: move 100px over animation
        } else {
            (0.0, 0.0)
        }
    }

    /// Is animation finished?
    pub fn is_finished(&self) -> bool {
        let scale_done = self.scale_anim.as_ref().map_or(true, |a| a.is_finished());
        let opacity_done = self.opacity_anim.as_ref().map_or(true, |a| a.is_finished());
        let pos_done = self.position_anim.as_ref().map_or(true, |a| a.is_finished());

        scale_done && opacity_done && pos_done
    }

    /// Progress (0.0–1.0)
    pub fn progress(&self) -> f32 {
        let mut max_progress: f32 = 0.0;
        if let Some(anim) = &self.scale_anim {
            max_progress = max_progress.max(anim.progress());
        }
        if let Some(anim) = &self.opacity_anim {
            max_progress = max_progress.max(anim.progress());
        }
        if let Some(anim) = &self.position_anim {
            max_progress = max_progress.max(anim.progress());
        }
        max_progress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_animation_config_default() {
        let config = WindowAnimationConfig::default();
        assert!(!config.open_duration.is_zero());
        assert!(!config.close_duration.is_zero());
    }

    #[test]
    fn test_window_open_animation() {
        let config = WindowAnimationConfig::default();
        let anim = WindowAnimation::open(config);
        assert!(anim.is_ok());
    }

    #[test]
    fn test_window_close_animation() {
        let config = WindowAnimationConfig::default();
        let mut anim = WindowAnimation::close(config);
        anim.start();
        assert_eq!(anim.state, WindowState::Closing);
    }

    #[test]
    fn test_window_minimize_animation() {
        let config = WindowAnimationConfig::default();
        let mut anim = WindowAnimation::minimize(config);
        anim.start();
        assert_eq!(anim.state, WindowState::Minimized);
    }

    #[test]
    fn test_window_animation_advance() {
        let config = WindowAnimationConfig::default();
        let mut anim = WindowAnimation::open(config).unwrap();

        for _ in 0..200 {
            anim.advance(Duration::from_millis(10));
            if anim.is_finished() {
                break;
            }
        }

        assert!(anim.is_finished());
        assert_eq!(anim.state, WindowState::Visible);
    }

    #[test]
    fn test_window_animation_scale() {
        let config = WindowAnimationConfig::default();
        let mut anim = WindowAnimation::open(config).unwrap();

        let initial_scale = anim.scale();
        anim.advance(Duration::from_millis(50));
        let mid_scale = anim.scale();

        // Scale should increase from 0.95 to 1.0
        assert!(mid_scale >= initial_scale);
    }

    #[test]
    fn test_window_animation_opacity() {
        let config = WindowAnimationConfig::default();
        let mut anim = WindowAnimation::open(config).unwrap();

        let initial_opacity = anim.opacity();
        assert_eq!(initial_opacity, 0.0);

        anim.advance(Duration::from_millis(50));
        let mid_opacity = anim.opacity();
        assert!(mid_opacity > 0.0);
    }
}
