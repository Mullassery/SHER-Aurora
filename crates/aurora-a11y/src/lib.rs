//! aurora-a11y
//!
//! Automated WCAG 2.1 accessibility auditing for Aurora's color system. Computes real
//! contrast ratios for every semantic token pairing across all shipped themes and
//! reports the actual conformance level reached, rather than asserting compliance
//! without measurement.

pub mod audit;
pub mod wcag;

pub use audit::{
    audit_all_themes, audit_theme, ContrastFinding, ThemeAuditReport, UiComponentFinding,
};
pub use wcag::{passes_ui_component_contrast, wcag_level, TextSize, WcagLevel};
