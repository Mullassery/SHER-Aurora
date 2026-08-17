//! aurora-web — planned web/WASM rendering backend
//!
//! # Status: not yet implemented
//!
//! This crate is a placeholder for a future web/WASM renderer for the
//! Aurora design system (e.g. compiling the token/typography/color/motion
//! logic layer to WASM and driving real DOM or `<canvas>` output),
//! analogous to `aurora-gtk` (which really does render Aurora widgets on
//! real GTK4 as of this pass). No WASM bindings or DOM rendering code exist
//! here yet — this crate currently contains no real code, and does not
//! claim rendering parity with `aurora-gtk`.
//!
//! A web backend is a legitimate, substantial platform-integration project
//! in its own right and is deliberately deferred rather than faked here.
