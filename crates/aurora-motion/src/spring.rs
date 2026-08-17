use crate::errors::{MotionError, MotionResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Spring physics configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpringConfig {
    pub mass: f32,     // Typically 1.0 (kg)
    pub tension: f32,  // Stiffness constant (N/m)
    pub friction: f32, // Damping coefficient (N·s/m)
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self::standard()
    }
}

impl SpringConfig {
    /// Standard Aurora spring (snappy, responsive)
    pub fn standard() -> Self {
        Self {
            mass: 1.0,
            tension: 280.0,
            friction: 60.0,
        }
    }

    /// Gentle spring (slow, bouncy)
    pub fn gentle() -> Self {
        Self {
            mass: 1.0,
            tension: 170.0,
            friction: 26.0,
        }
    }

    /// Snappy spring (fast, minimal overshoot)
    pub fn snappy() -> Self {
        Self {
            mass: 1.0,
            tension: 350.0,
            friction: 40.0,
        }
    }

    /// Stiff spring (instant, no overshoot)
    pub fn stiff() -> Self {
        Self {
            mass: 1.0,
            tension: 500.0,
            friction: 80.0,
        }
    }

    /// Bouncy spring (playful, overshoot)
    pub fn bouncy() -> Self {
        Self {
            mass: 1.0,
            tension: 200.0,
            friction: 15.0,
        }
    }

    pub fn validate(&self) -> MotionResult<()> {
        if self.mass <= 0.0 {
            return Err(MotionError::InvalidSpringParameter(
                "Mass must be > 0".to_string(),
            ));
        }
        if self.tension <= 0.0 {
            return Err(MotionError::InvalidSpringParameter(
                "Tension must be > 0".to_string(),
            ));
        }
        if self.friction < 0.0 {
            return Err(MotionError::InvalidSpringParameter(
                "Friction must be >= 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Spring animation state
#[derive(Debug, Clone, Copy)]
pub struct SpringState {
    pub position: f32,     // Current position
    pub velocity: f32,     // Current velocity
    pub acceleration: f32, // Current acceleration
}

impl SpringState {
    pub fn new(position: f32, velocity: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration: 0.0,
        }
    }

    /// Resting position (no motion)
    pub fn rest() -> Self {
        Self {
            position: 0.0,
            velocity: 0.0,
            acceleration: 0.0,
        }
    }

    /// Is the spring effectively at rest?
    pub fn is_at_rest(&self, epsilon: f32) -> bool {
        self.velocity.abs() < epsilon && self.acceleration.abs() < epsilon
    }
}

/// Spring animator — simulates spring physics
#[derive(Debug, Clone)]
pub struct SpringAnimator {
    config: SpringConfig,
    start: f32,
    end: f32,
    state: SpringState,
    elapsed: Duration,
    max_time: Duration,
    finished: bool,
}

impl SpringAnimator {
    pub fn new(config: SpringConfig, start: f32, end: f32) -> MotionResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            start,
            end,
            state: SpringState::new(start, 0.0),
            elapsed: Duration::ZERO,
            max_time: Duration::from_secs(10), // Max animation time
            finished: false,
        })
    }

    /// Advance animation by time delta
    pub fn advance(&mut self, delta: Duration) {
        if self.finished {
            return;
        }

        self.elapsed += delta;
        let dt = delta.as_secs_f32();

        // Spring force: F = -k(x - x_target)
        let displacement = self.state.position - self.end;
        let spring_force = -self.config.tension * displacement;

        // Damping force: F = -c * v
        let damping_force = -self.config.friction * self.state.velocity;

        // Total acceleration: a = (F_spring + F_damping) / m
        self.state.acceleration = (spring_force + damping_force) / self.config.mass;

        // Update velocity: v = v + a * dt
        self.state.velocity += self.state.acceleration * dt;

        // Update position: x = x + v * dt
        self.state.position += self.state.velocity * dt;

        // Check if animation is complete (at rest near target)
        const EPSILON: f32 = 0.001; // 0.1% tolerance
        if self.state.is_at_rest(EPSILON) && (self.state.position - self.end).abs() < EPSILON {
            self.state.position = self.end;
            self.state.velocity = 0.0;
            self.finished = true;
        }

        // Timeout after max_time
        if self.elapsed > self.max_time {
            self.state.position = self.end;
            self.finished = true;
        }
    }

    /// Get current interpolated value (0.0–1.0)
    pub fn current(&self) -> f32 {
        self.state.position
    }

    /// Get current value in range [start, end]
    pub fn current_in_range(&self) -> f32 {
        self.start + self.state.position * (self.end - self.start)
    }

    /// Is animation finished?
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Time elapsed
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed.as_millis() as u64
    }

    /// Progress (0.0–1.0)
    pub fn progress(&self) -> f32 {
        if self.finished {
            1.0
        } else {
            let normalized = self.elapsed.as_secs_f32() / (self.max_time.as_secs_f32() * 0.5); // Estimate
            normalized.min(1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_config_standard() {
        let config = SpringConfig::standard();
        assert_eq!(config.tension, 280.0);
        assert_eq!(config.friction, 60.0);
    }

    #[test]
    fn test_spring_config_presets() {
        let gentle = SpringConfig::gentle();
        let snappy = SpringConfig::snappy();
        assert!(snappy.tension > gentle.tension);
        assert!(snappy.friction > gentle.friction); // Snappy has more damping, not less
    }

    #[test]
    fn test_spring_config_validation() {
        let invalid = SpringConfig {
            mass: -1.0,
            tension: 280.0,
            friction: 60.0,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_spring_state_rest() {
        let state = SpringState::rest();
        assert_eq!(state.position, 0.0);
        assert_eq!(state.velocity, 0.0);
    }

    #[test]
    fn test_spring_state_at_rest() {
        let state = SpringState::new(0.0, 0.0);
        assert!(state.is_at_rest(0.01));
    }

    #[test]
    fn test_spring_animator_creation() {
        let animator = SpringAnimator::new(SpringConfig::standard(), 0.0, 1.0);
        assert!(animator.is_ok());
    }

    #[test]
    fn test_spring_animator_animation() {
        let mut animator = SpringAnimator::new(SpringConfig::standard(), 0.0, 1.0).unwrap();

        // Advance animation
        for _ in 0..1000 {
            animator.advance(Duration::from_millis(5));
            if animator.is_finished() {
                break;
            }
        }

        // Should be finished and at end position (within 0.1% tolerance)
        assert!(animator.is_finished());
        assert!((animator.current() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_spring_animator_progress() {
        let mut animator = SpringAnimator::new(SpringConfig::standard(), 0.0, 1.0).unwrap();
        let initial_progress = animator.progress();
        assert!((0.0..=1.0).contains(&initial_progress));

        animator.advance(Duration::from_millis(100));
        let mid_progress = animator.progress();
        assert!(mid_progress > initial_progress);
    }

    #[test]
    fn test_spring_animator_with_initial_velocity() {
        let mut animator = SpringAnimator::new(SpringConfig::standard(), 0.0, 1.0).unwrap();
        animator.state.velocity = 0.5; // Initial velocity

        animator.advance(Duration::from_millis(10));
        assert!(animator.state.position > 0.0);
    }
}
