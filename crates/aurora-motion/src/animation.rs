use crate::easing::EasingFunction;
use crate::spring::{SpringAnimator, SpringConfig};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Animation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnimationType {
    Spring,  // Spring physics animation
    Tween,   // Linear interpolation with easing
}

/// Animation state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnimationState {
    Idle,       // Not started
    Running,    // In progress
    Paused,     // Paused
    Finished,   // Complete
}

/// Complete animation definition
#[derive(Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub animation_type: AnimationType,
    pub start: f32,
    pub end: f32,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub spring_config: Option<SpringConfig>,
    pub delay: Duration,
    pub state: AnimationState,
    pub elapsed: Duration,
    pub spring_animator: Option<SpringAnimator>,
    pub allow_gpu_acceleration: bool,
}

impl Animation {
    /// Create a spring animation
    pub fn spring(
        name: impl Into<String>,
        start: f32,
        end: f32,
        config: SpringConfig,
    ) -> crate::errors::MotionResult<Self> {
        let spring_animator = SpringAnimator::new(config, start, end)?;

        Ok(Self {
            name: name.into(),
            animation_type: AnimationType::Spring,
            start,
            end,
            duration: Duration::from_secs(10), // Max time
            easing: EasingFunction::Linear,
            spring_config: Some(config),
            delay: Duration::ZERO,
            state: AnimationState::Idle,
            elapsed: Duration::ZERO,
            spring_animator: Some(spring_animator),
            allow_gpu_acceleration: true,
        })
    }

    /// Create a tween animation
    pub fn tween(
        name: impl Into<String>,
        start: f32,
        end: f32,
        duration: Duration,
        easing: EasingFunction,
    ) -> Self {
        Self {
            name: name.into(),
            animation_type: AnimationType::Tween,
            start,
            end,
            duration,
            easing,
            spring_config: None,
            delay: Duration::ZERO,
            state: AnimationState::Idle,
            elapsed: Duration::ZERO,
            spring_animator: None,
            allow_gpu_acceleration: true,
        }
    }

    /// Start the animation
    pub fn start(&mut self) {
        if self.delay > Duration::ZERO {
            self.state = AnimationState::Idle;
        } else {
            self.state = AnimationState::Running;
        }
        self.elapsed = Duration::ZERO;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Running;
        }
    }

    /// Set delay before animation starts
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Allow GPU acceleration (transform, opacity)
    pub fn with_gpu_acceleration(mut self, allow: bool) -> Self {
        self.allow_gpu_acceleration = allow;
        self
    }

    /// Advance animation by time delta
    pub fn advance(&mut self, delta: Duration) {
        match self.state {
            AnimationState::Idle => {
                self.elapsed += delta;
                if self.elapsed >= self.delay {
                    self.state = AnimationState::Running;
                    self.elapsed = Duration::ZERO;
                }
            }
            AnimationState::Running => {
                self.elapsed += delta;

                if self.animation_type == AnimationType::Spring {
                    if let Some(ref mut animator) = self.spring_animator {
                        animator.advance(delta);
                        if animator.is_finished() {
                            self.state = AnimationState::Finished;
                        }
                    }
                } else if self.elapsed >= self.duration {
                    self.state = AnimationState::Finished;
                }
            }
            AnimationState::Paused | AnimationState::Finished => {}
        }
    }

    /// Get current interpolated value (0.0–1.0)
    pub fn current(&self) -> f32 {
        match self.state {
            AnimationState::Idle | AnimationState::Paused => {
                if self.animation_type == AnimationType::Spring {
                    if let Some(ref animator) = self.spring_animator {
                        return animator.current();
                    }
                }
                0.0
            }
            AnimationState::Running => {
                if self.animation_type == AnimationType::Spring {
                    if let Some(ref animator) = self.spring_animator {
                        return animator.current();
                    }
                }

                let progress = (self.elapsed.as_secs_f32() / self.duration.as_secs_f32())
                    .min(1.0);
                self.easing.evaluate(progress)
            }
            AnimationState::Finished => 1.0,
        }
    }

    /// Get current value in range [start, end]
    pub fn current_in_range(&self) -> f32 {
        self.start + self.current() * (self.end - self.start)
    }

    /// Is animation finished?
    pub fn is_finished(&self) -> bool {
        self.state == AnimationState::Finished
    }

    /// Progress (0.0–1.0)
    pub fn progress(&self) -> f32 {
        match self.state {
            AnimationState::Idle => 0.0,
            AnimationState::Running => {
                if self.animation_type == AnimationType::Spring {
                    if let Some(ref animator) = self.spring_animator {
                        return animator.progress();
                    }
                }
                (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
            }
            AnimationState::Paused => {
                (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
            }
            AnimationState::Finished => 1.0,
        }
    }

    /// Elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed.as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_animation_creation() {
        let anim = Animation::spring("test", 0.0, 1.0, SpringConfig::standard());
        assert!(anim.is_ok());
    }

    #[test]
    fn test_tween_animation_creation() {
        let anim =
            Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::EaseOut);
        assert_eq!(anim.name, "test");
        assert_eq!(anim.start, 0.0);
        assert_eq!(anim.end, 1.0);
    }

    #[test]
    fn test_animation_start() {
        let mut anim =
            Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::Linear);
        assert_eq!(anim.state, AnimationState::Idle);
        anim.start();
        assert_eq!(anim.state, AnimationState::Running);
    }

    #[test]
    fn test_animation_pause_resume() {
        let mut anim =
            Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::Linear);
        anim.start();
        anim.pause();
        assert_eq!(anim.state, AnimationState::Paused);
        anim.resume();
        assert_eq!(anim.state, AnimationState::Running);
    }

    #[test]
    fn test_animation_advance() {
        let mut anim =
            Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::Linear);
        anim.start();
        anim.advance(Duration::from_millis(100));
        assert!(anim.progress() > 0.0);
        assert!(anim.progress() < 1.0);
    }

    #[test]
    fn test_animation_completion() {
        let mut anim =
            Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::Linear);
        anim.start();

        for _ in 0..10 {
            anim.advance(Duration::from_millis(30));
            if anim.is_finished() {
                break;
            }
        }

        assert!(anim.is_finished());
        assert_eq!(anim.progress(), 1.0);
    }

    #[test]
    fn test_animation_with_delay() {
        let mut anim = Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::Linear)
            .with_delay(Duration::from_millis(100));
        anim.start();

        // Advance less than delay
        anim.advance(Duration::from_millis(50));
        assert_eq!(anim.state, AnimationState::Idle);

        // Advance past delay
        anim.advance(Duration::from_millis(60));
        assert_eq!(anim.state, AnimationState::Running);
    }

    #[test]
    fn test_animation_current_in_range() {
        let mut anim =
            Animation::tween("test", 10.0, 20.0, Duration::from_millis(200), EasingFunction::Linear);
        anim.start();
        anim.advance(Duration::from_millis(100));

        let current = anim.current_in_range();
        assert!(current > 10.0 && current < 20.0);
    }

    #[test]
    fn test_animation_gpu_acceleration() {
        let anim = Animation::tween("test", 0.0, 1.0, Duration::from_millis(200), EasingFunction::Linear)
            .with_gpu_acceleration(false);
        assert!(!anim.allow_gpu_acceleration);
    }
}
