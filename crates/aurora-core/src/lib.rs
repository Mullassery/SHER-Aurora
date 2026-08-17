//! aurora-core — planned unified API over all Aurora subsystems
//!
//! # Status: not yet implemented
//!
//! This crate is a placeholder for a future unified `Aurora` facade that
//! would expose `aurora-tokens`, `aurora-typography`, `aurora-color`,
//! `aurora-motion`, `aurora-icons`, `aurora-sound`, and `aurora-a11y`
//! through one coherent API (see `docs/ARCHITECTURE.md` for the sketched
//! design). As of this writing it contains no real code — do not depend on
//! it expecting working functionality. Each subsystem crate is independently
//! usable today; consume those directly until this facade lands.
//!
//! This is intentionally scoped as deferred, follow-up work rather than
//! implemented in this pass, the same way the `aurora-qt` and `aurora-web`
//! rendering-backend crates are deferred — building a real, useful facade
//! over seven independently-evolving subsystems is a substantial project of
//! its own, not something to stub out convincingly just to claim it's done.
