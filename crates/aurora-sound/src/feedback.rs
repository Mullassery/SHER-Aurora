use crate::sound::Sound;

/// User interaction feedback types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feedback {
    Success,
    Error,
    Warning,
    Notification,
    Click,
    Hover,
}

/// Sound feedback mapping
#[derive(Debug, Clone)]
pub struct SoundFeedback {
    feedback: Feedback,
    sound: Sound,
}

impl SoundFeedback {
    /// Create sound feedback for an interaction
    pub fn new(feedback: Feedback) -> Self {
        let sound = match feedback {
            Feedback::Success => Sound::Success,
            Feedback::Error => Sound::Error,
            Feedback::Warning => Sound::Warning,
            Feedback::Notification => Sound::Notification,
            Feedback::Click => Sound::Click,
            Feedback::Hover => Sound::Hover,
        };

        Self { feedback, sound }
    }

    /// Get feedback type
    pub fn feedback(&self) -> Feedback {
        self.feedback
    }

    /// Get associated sound
    pub fn sound(&self) -> Sound {
        self.sound
    }

    /// Get visual feedback text (for accessibility)
    pub fn visual_feedback(&self) -> &'static str {
        match self.feedback {
            Feedback::Success => "Success",
            Feedback::Error => "Error",
            Feedback::Warning => "Warning",
            Feedback::Notification => "Notification",
            Feedback::Click => "Button clicked",
            Feedback::Hover => "Hovering",
        }
    }

    /// Get accessibility announcement
    pub fn a11y_announcement(&self) -> &'static str {
        match self.feedback {
            Feedback::Success => "Action completed successfully",
            Feedback::Error => "Error occurred",
            Feedback::Warning => "Warning: please review",
            Feedback::Notification => "New notification",
            Feedback::Click => "Button activated",
            Feedback::Hover => "Element focused",
        }
    }
}

impl Default for SoundFeedback {
    fn default() -> Self {
        Self::new(Feedback::Click)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_feedback_creation() {
        let feedback = SoundFeedback::new(Feedback::Success);
        assert_eq!(feedback.feedback(), Feedback::Success);
        assert_eq!(feedback.sound(), Sound::Success);
    }

    #[test]
    fn test_visual_feedback() {
        let feedback = SoundFeedback::new(Feedback::Success);
        assert_eq!(feedback.visual_feedback(), "Success");
    }

    #[test]
    fn test_a11y_announcement() {
        let feedback = SoundFeedback::new(Feedback::Success);
        assert_eq!(feedback.a11y_announcement(), "Action completed successfully");
    }

    #[test]
    fn test_all_feedback_types() {
        let feedbacks = vec![
            Feedback::Success,
            Feedback::Error,
            Feedback::Warning,
            Feedback::Notification,
            Feedback::Click,
            Feedback::Hover,
        ];

        for feedback_type in feedbacks {
            let feedback = SoundFeedback::new(feedback_type);
            assert!(!feedback.visual_feedback().is_empty());
            assert!(!feedback.a11y_announcement().is_empty());
        }
    }

    #[test]
    fn test_default_feedback() {
        let feedback = SoundFeedback::default();
        assert_eq!(feedback.feedback(), Feedback::Click);
    }
}
