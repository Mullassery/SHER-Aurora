//! Aurora Music - Media Player Example
//!
//! Demonstrates Aurora components in a music player application.

/// Music track
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    id: u32,
    title: String,
    artist: String,
    duration: u32, // seconds
}

impl Track {
    /// Create a new track
    pub fn new(id: u32, title: &str, artist: &str, duration: u32) -> Self {
        Self {
            id,
            title: title.to_string(),
            artist: artist.to_string(),
            duration,
        }
    }

    /// Get track ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get track title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get track artist
    pub fn artist(&self) -> &str {
        &self.artist
    }

    /// Get track duration
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Get formatted duration (MM:SS)
    pub fn formatted_duration(&self) -> String {
        let minutes = self.duration / 60;
        let seconds = self.duration % 60;
        format!("{}:{:02}", minutes, seconds)
    }
}

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

/// Aurora music player
pub struct AuroraMusicPlayer {
    title: String,
    tracks: Vec<Track>,
    current_track: Option<usize>,
    playback_state: PlaybackState,
    current_time: u32,
    volume: f32,
}

impl AuroraMusicPlayer {
    /// Create a new music player
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            tracks: Vec::new(),
            current_track: None,
            playback_state: PlaybackState::Stopped,
            current_time: 0,
            volume: 0.8,
        }
    }

    /// Get player title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Add track to playlist
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    /// Get all tracks
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// Get current track
    pub fn current_track(&self) -> Option<&Track> {
        self.current_track.and_then(|idx| self.tracks.get(idx))
    }

    /// Play a track by index
    pub fn play(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.current_track = Some(index);
            self.playback_state = PlaybackState::Playing;
            self.current_time = 0;
        }
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.playback_state == PlaybackState::Playing {
            self.playback_state = PlaybackState::Paused;
        }
    }

    /// Resume playback
    pub fn resume(&mut self) {
        if self.playback_state == PlaybackState::Paused {
            self.playback_state = PlaybackState::Playing;
        }
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.current_time = 0;
    }

    /// Get playback state
    pub fn playback_state(&self) -> PlaybackState {
        self.playback_state
    }

    /// Next track
    pub fn next(&mut self) {
        if let Some(idx) = self.current_track {
            if idx + 1 < self.tracks.len() {
                self.play(idx + 1);
            }
        }
    }

    /// Previous track
    pub fn prev(&mut self) {
        if let Some(idx) = self.current_track {
            if idx > 0 {
                self.play(idx - 1);
            }
        }
    }

    /// Set current time (in seconds)
    pub fn seek(&mut self, time: u32) {
        if let Some(track) = self.current_track() {
            self.current_time = time.min(track.duration());
        }
    }

    /// Get current playback time
    pub fn current_time(&self) -> u32 {
        self.current_time
    }

    /// Set volume (0.0-1.0)
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Get volume
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Get track count
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

impl Default for AuroraMusicPlayer {
    fn default() -> Self {
        Self::new("Aurora Music")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = AuroraMusicPlayer::new("My Music");
        assert_eq!(player.title(), "My Music");
        assert_eq!(player.track_count(), 0);
    }

    #[test]
    fn test_add_track() {
        let mut player = AuroraMusicPlayer::new("Player");
        let track = Track::new(1, "Song", "Artist", 180);
        player.add_track(track);
        assert_eq!(player.track_count(), 1);
    }

    #[test]
    fn test_track_duration_format() {
        let track = Track::new(1, "Song", "Artist", 185);
        assert_eq!(track.formatted_duration(), "3:05");
    }

    #[test]
    fn test_play_track() {
        let mut player = AuroraMusicPlayer::new("Player");
        let track = Track::new(1, "Song", "Artist", 180);
        player.add_track(track);
        player.play(0);

        assert_eq!(player.playback_state(), PlaybackState::Playing);
        assert_eq!(player.current_track().unwrap().title(), "Song");
    }

    #[test]
    fn test_pause_resume() {
        let mut player = AuroraMusicPlayer::new("Player");
        let track = Track::new(1, "Song", "Artist", 180);
        player.add_track(track);
        player.play(0);

        player.pause();
        assert_eq!(player.playback_state(), PlaybackState::Paused);

        player.resume();
        assert_eq!(player.playback_state(), PlaybackState::Playing);
    }

    #[test]
    fn test_stop() {
        let mut player = AuroraMusicPlayer::new("Player");
        let track = Track::new(1, "Song", "Artist", 180);
        player.add_track(track);
        player.play(0);
        player.stop();

        assert_eq!(player.playback_state(), PlaybackState::Stopped);
        assert_eq!(player.current_time(), 0);
    }

    #[test]
    fn test_next_previous() {
        let mut player = AuroraMusicPlayer::new("Player");
        player.add_track(Track::new(1, "Song 1", "Artist", 180));
        player.add_track(Track::new(2, "Song 2", "Artist", 200));

        player.play(0);
        player.next();
        assert_eq!(player.current_track().unwrap().title(), "Song 2");

        player.prev();
        assert_eq!(player.current_track().unwrap().title(), "Song 1");
    }

    #[test]
    fn test_seek() {
        let mut player = AuroraMusicPlayer::new("Player");
        let track = Track::new(1, "Song", "Artist", 180);
        player.add_track(track);
        player.play(0);

        player.seek(60);
        assert_eq!(player.current_time(), 60);
    }

    #[test]
    fn test_volume() {
        let mut player = AuroraMusicPlayer::new("Player");
        player.set_volume(0.5);
        assert_eq!(player.volume(), 0.5);

        player.set_volume(2.0);
        assert_eq!(player.volume(), 1.0);
    }

    #[test]
    fn test_default() {
        let player = AuroraMusicPlayer::default();
        assert_eq!(player.title(), "Aurora Music");
    }
}
