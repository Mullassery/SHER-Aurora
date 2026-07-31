//! Aurora Motion Engine
//!
//! Spring physics-based animation system with:
//! - Spring physics (mass, tension, friction)
//! - Tween animations with easing
//! - Animation timeline management
//! - GPU acceleration hints
//! - Keyframe support

pub mod errors;
pub mod spring;
pub mod easing;
pub mod animation;

pub use errors::{MotionError, MotionResult};
pub use spring::{SpringConfig, SpringState, SpringAnimator};
pub use easing::EasingFunction;
pub use animation::{Animation, AnimationType, AnimationState};

/// Motion engine — manages all animations
#[derive(Debug)]
pub struct MotionEngine {
    animations: Vec<Animation>,
}

impl MotionEngine {
    /// Create a new motion engine
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }

    /// Add a spring animation
    pub fn add_spring(
        &mut self,
        name: impl Into<String>,
        start: f32,
        end: f32,
        config: SpringConfig,
    ) -> MotionResult<()> {
        let anim = Animation::spring(name, start, end, config)?;
        self.animations.push(anim);
        Ok(())
    }

    /// Add a tween animation
    pub fn add_tween(
        &mut self,
        name: impl Into<String>,
        start: f32,
        end: f32,
        duration: std::time::Duration,
        easing: EasingFunction,
    ) {
        let anim = Animation::tween(name, start, end, duration, easing);
        self.animations.push(anim);
    }

    /// Get animation by name
    pub fn get(&self, name: &str) -> Option<&Animation> {
        self.animations.iter().find(|a| a.name == name)
    }

    /// Get mutable animation by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.animations.iter_mut().find(|a| a.name == name)
    }

    /// Start an animation
    pub fn start(&mut self, name: &str) -> MotionResult<()> {
        self.get_mut(name)
            .ok_or_else(|| MotionError::AnimationNotFound(name.to_string()))?
            .start();
        Ok(())
    }

    /// Pause an animation
    pub fn pause(&mut self, name: &str) -> MotionResult<()> {
        self.get_mut(name)
            .ok_or_else(|| MotionError::AnimationNotFound(name.to_string()))?
            .pause();
        Ok(())
    }

    /// Resume an animation
    pub fn resume(&mut self, name: &str) -> MotionResult<()> {
        self.get_mut(name)
            .ok_or_else(|| MotionError::AnimationNotFound(name.to_string()))?
            .resume();
        Ok(())
    }

    /// Advance all animations
    pub fn advance(&mut self, delta: std::time::Duration) {
        for anim in &mut self.animations {
            anim.advance(delta);
        }
    }

    /// Remove finished animations
    pub fn cleanup_finished(&mut self) {
        self.animations.retain(|a| !a.is_finished());
    }

    /// Number of active animations
    pub fn active_count(&self) -> usize {
        self.animations
            .iter()
            .filter(|a| !a.is_finished())
            .count()
    }

    /// Are all animations finished?
    pub fn all_finished(&self) -> bool {
        self.animations.iter().all(|a| a.is_finished())
    }
}

impl Default for MotionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_engine_creation() {
        let engine = MotionEngine::new();
        assert_eq!(engine.active_count(), 0);
    }

    #[test]
    fn test_motion_engine_add_tween() {
        let mut engine = MotionEngine::new();
        engine.add_tween(
            "test",
            0.0,
            1.0,
            std::time::Duration::from_millis(200),
            EasingFunction::Linear,
        );
        assert_eq!(engine.active_count(), 1);
    }

    #[test]
    fn test_motion_engine_add_spring() {
        let mut engine = MotionEngine::new();
        engine
            .add_spring("test", 0.0, 1.0, SpringConfig::standard())
            .unwrap();
        assert_eq!(engine.active_count(), 1);
    }

    #[test]
    fn test_motion_engine_get() {
        let mut engine = MotionEngine::new();
        engine.add_tween(
            "test",
            0.0,
            1.0,
            std::time::Duration::from_millis(200),
            EasingFunction::Linear,
        );
        assert!(engine.get("test").is_some());
        assert!(engine.get("nonexistent").is_none());
    }

    #[test]
    fn test_motion_engine_start_pause_resume() {
        let mut engine = MotionEngine::new();
        engine.add_tween(
            "test",
            0.0,
            1.0,
            std::time::Duration::from_millis(200),
            EasingFunction::Linear,
        );
        engine.start("test").unwrap();
        engine.pause("test").unwrap();
        engine.resume("test").unwrap();
    }

    #[test]
    fn test_motion_engine_advance() {
        let mut engine = MotionEngine::new();
        engine.add_tween(
            "test",
            0.0,
            1.0,
            std::time::Duration::from_millis(200),
            EasingFunction::Linear,
        );
        engine.start("test").unwrap();
        engine.advance(std::time::Duration::from_millis(100));
        let anim = engine.get("test").unwrap();
        assert!(anim.progress() > 0.0);
    }

    #[test]
    fn test_motion_engine_cleanup() {
        let mut engine = MotionEngine::new();
        engine.add_tween(
            "test",
            0.0,
            1.0,
            std::time::Duration::from_millis(50),
            EasingFunction::Linear,
        );
        engine.start("test").unwrap();

        for _ in 0..5 {
            engine.advance(std::time::Duration::from_millis(20));
        }

        assert!(engine.all_finished());
        engine.cleanup_finished();
        assert_eq!(engine.active_count(), 0);
    }
}

