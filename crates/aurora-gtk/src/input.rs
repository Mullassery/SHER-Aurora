//! Input event dispatcher for SHER-Aurora
//!
//! Bridges raw hardware/input events from SHER-Input into GTK4/GLib main loop.
//! This module is generic over a source to avoid hard coupling; enable the
//! `sher-input` feature to use [`sher_input_core`] types directly.

use std::sync::{Arc, Mutex};
use std::time::Duration;

#[cfg(feature = "sher-input")]
pub use sher_input_core::{InputRingBuffer, RawInputEvent as SherRawInputEvent};

/// A raw input event (hardware level). Duplicate of SHER-Input's raw event to
/// allow use without a cross-repo dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RawInputEvent {
    pub event_type: u16, // 1=EV_KEY, 2=EV_REL, 3=EV_ABS
    pub code: u16,
    pub value: i32,
    pub timestamp: u64,
}

impl Default for RawInputEvent {
    fn default() -> Self {
        Self {
            event_type: 0,
            code: 0,
            value: 0,
            timestamp: 0,
        }
    }
}

/// Source of raw input events.
pub trait InputSource: Send + Sync {
    fn try_pop(&self) -> Option<RawInputEvent>;
}

/// Sink for dispatched events.
pub trait InputSink: Send + Sync {
    fn send_key(&self, window_id: Option<u64>, code: u16, pressed: bool);
    fn send_pointer_move(&self, window_id: Option<u64>, x: i32, y: i32);
    fn send_abs(&self, axis: u16, value: i32);
}

/// Default no-op sink for tests/integration.
#[derive(Debug, Default)]
pub struct NoopSink;

impl InputSink for NoopSink {
    fn send_key(&self, _window_id: Option<u64>, _code: u16, _pressed: bool) {}
    fn send_pointer_move(&self, _window_id: Option<u64>, _x: i32, _y: i32) {}
    fn send_abs(&self, _axis: u16, _value: i32) {}
}

/// Adapter from SHER-Input ring buffer of RawInputEvent.
#[cfg(feature = "sher-input")]
pub struct SherInputSource<const N: usize> {
    ring: Arc<InputRingBuffer<sher_input_core::RawInputEvent, N>>,
}

#[cfg(feature = "sher-input")]
impl<const N: usize> SherInputSource<N> {
    pub fn new(ring: Arc<InputRingBuffer<sher_input_core::RawInputEvent, N>>) -> Self {
        Self { ring }
    }
}

#[cfg(feature = "sher-input")]
impl<const N: usize> InputSource for SherInputSource<N> {
    fn try_pop(&self) -> Option<RawInputEvent> {
        self.ring.pop().map(|e| RawInputEvent {
            event_type: e.event_type,
            code: e.code,
            value: e.value,
            timestamp: e.timestamp,
        })
    }
}

#[cfg(feature = "sher-input")]
impl<const N: usize> InputSource for InputRingBuffer<sher_input_core::RawInputEvent, N> {
    fn try_pop(&self) -> Option<RawInputEvent> {
        self.pop().map(|e| RawInputEvent {
            event_type: e.event_type,
            code: e.code,
            value: e.value,
            timestamp: e.timestamp,
        })
    }
}

/// Dispatcher that routes raw events to focused surfaces.
#[derive(Clone)]
pub struct AuroraInputDispatcher<S = NoopSink> {
    source: Arc<Mutex<dyn InputSource>>,
    sink: S,
    active_window_id: Option<u64>,
    pointer_x: i32,
    pointer_y: i32,
}

impl<S: InputSink> AuroraInputDispatcher<S> {
    pub fn new(source: Arc<Mutex<dyn InputSource>>, sink: S) -> Self {
        Self {
            source,
            sink,
            active_window_id: None,
            pointer_x: 0,
            pointer_y: 0,
        }
    }

    pub fn set_active_window(&mut self, window_id: Option<u64>) {
        self.active_window_id = window_id;
    }

    pub fn active_window(&self) -> Option<u64> {
        self.active_window_id
    }

    /// Poll and dispatch all pending events.
    pub fn poll_and_dispatch(&mut self) {
        loop {
            let event = {
                let src = self.source.lock().unwrap();
                src.try_pop()
            };
            match event {
                Some(e) => self.route_event(e),
                None => break,
            }
        }
    }

    fn route_event(&mut self, event: RawInputEvent) {
        match event.event_type {
            // EV_KEY
            1 => {
                let pressed = event.value == 1;
                self.sink
                    .send_key(self.active_window_id, event.code, pressed);
            }
            // EV_ABS
            3 => {
                // Update coordinates; convention varies by backend
                self.sink.send_abs(event.code, event.value);
                // Track common axes if needed (ABS_X/ABS_Y)
                if event.code == 0x00 {
                    // ABS_X
                    self.pointer_x = event.value;
                } else if event.code == 0x01 {
                    // ABS_Y
                    self.pointer_y = event.value;
                }
                self.sink
                    .send_pointer_move(self.active_window_id, self.pointer_x, self.pointer_y);
            }
            // EV_REL could be added as needed
            2 => {
                // Relative motion not routed here by default
            }
            _ => {}
        }
    }

    /// Attach to GLib main loop as a timeout source (~500Hz default).
    #[cfg(feature = "glib")]
    pub fn attach_to_glib(&self, interval: Duration) -> glib::SourceId {
        let dispatcher = self.clone();
        let interval_ms = interval.as_millis() as u32;
        glib::timeout_add_local(std::time::Duration::from_millis(interval_ms), move || {
            let mut d = dispatcher.clone();
            d.poll_and_dispatch();
            glib::ControlFlow::Continue
        })
    }
}

#[cfg(feature = "glib")]
impl<S: InputSink + 'static> AuroraInputDispatcher<S> {
    /// Convenience constructor using glib timeout.
    pub fn spawn_glib(dispatcher: AuroraInputDispatcher<S>, interval: Duration) -> glib::SourceId {
        dispatcher.attach_to_glib(interval)
    }
}

/// Spawn a background worker thread that polls at ~500Hz.
pub fn spawn_aurora_input_worker<S: InputSink + Send + Sync + 'static>(
    dispatcher: AuroraInputDispatcher<S>,
    poll_interval: Duration,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut d = dispatcher;
        loop {
            d.poll_and_dispatch();
            std::thread::sleep(poll_interval);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    #[derive(Clone, Default)]
    struct TestSource {
        queue: Arc<Mutex<VecDeque<RawInputEvent>>>,
    }

    impl InputSource for TestSource {
        fn try_pop(&self) -> Option<RawInputEvent> {
            self.queue.lock().unwrap().pop_front()
        }
    }

    impl TestSource {
        fn push(&self, e: RawInputEvent) {
            self.queue.lock().unwrap().push_back(e);
        }
    }

    #[derive(Clone, Default)]
    struct TestSink {
        keys: Arc<Mutex<Vec<(Option<u64>, u16, bool)>>>,
    }

    impl InputSink for TestSink {
        fn send_key(&self, window_id: Option<u64>, code: u16, pressed: bool) {
            self.keys.lock().unwrap().push((window_id, code, pressed));
        }
        fn send_pointer_move(&self, window_id: Option<u64>, x: i32, y: i32) {
            let _ = (window_id, x, y);
        }
        fn send_abs(&self, axis: u16, value: i32) {
            let _ = (axis, value);
        }
    }

    #[test]
    fn routes_key_events_to_active_window() {
        let src = TestSource::default();
        let sink = TestSink::default();
        let mut d = AuroraInputDispatcher::new(Arc::new(Mutex::new(src.clone())), sink.clone());
        d.set_active_window(Some(42));
        src.push(RawInputEvent {
            event_type: 1,
            code: 30,
            value: 1,
            timestamp: 0,
        });
        d.poll_and_dispatch();
        assert_eq!(sink.keys.lock().unwrap().len(), 1);
        assert_eq!(sink.keys.lock().unwrap()[0], (Some(42), 30, true));
    }

    #[test]
    fn drains_all_pending() {
        let src = TestSource::default();
        let sink = TestSink::default();
        let mut d = AuroraInputDispatcher::new(Arc::new(Mutex::new(src.clone())), sink.clone());
        for i in 0..5 {
            src.push(RawInputEvent {
                event_type: 1,
                code: i as u16,
                value: 1,
                timestamp: i,
            });
        }
        d.poll_and_dispatch();
        assert_eq!(sink.keys.lock().unwrap().len(), 5);
    }
}
