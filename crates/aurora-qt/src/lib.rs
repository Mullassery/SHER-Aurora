//! aurora-qt — planned Qt/QML rendering backend
//!
//! # Status: not yet implemented
//!
//! This crate is a placeholder for a future Qt/QML renderer for the Aurora
//! design system, analogous to `aurora-gtk` (which really does render
//! Aurora widgets on real GTK4 as of this pass). No Qt bindings, widgets, or
//! rendering code exist here yet — this crate currently contains no real
//! code, and does not claim rendering parity with `aurora-gtk`.
//!
//! A Qt backend is a legitimate, substantial platform-integration project in
//! its own right (choosing between `cxx-qt` / `qmetaobject-rs` bindings,
//! building an equivalent widget layer, etc.) and is deliberately deferred
//! rather than faked here.
