//! Automated contrast auditing across Aurora's real color tokens and themes.
//!
//! Walks every semantically meaningful foreground/background pairing in
//! [`ColorSystem`] (text-bearing surfaces, "on-brand-color" text sitting on a tonal
//! container, and non-text UI components) and computes the actual WCAG conformance
//! level for each pair, for every shipped theme.

use aurora_color::{Color, ColorSystem, ThemeName};

use crate::wcag::{passes_ui_component_contrast, wcag_level, TextSize, WcagLevel};

/// One evaluated foreground/background pairing.
#[derive(Debug, Clone)]
pub struct ContrastFinding {
    pub pair: &'static str,
    pub foreground_hex: String,
    pub background_hex: String,
    pub ratio: f32,
    pub level: WcagLevel,
}

/// One evaluated non-text UI component pairing (SC 1.4.11).
#[derive(Debug, Clone)]
pub struct UiComponentFinding {
    pub pair: &'static str,
    pub ratio: f32,
    pub passes: bool,
}

/// Full audit result for a single theme at a single text size.
#[derive(Debug, Clone)]
pub struct ThemeAuditReport {
    pub theme: ThemeName,
    pub size: TextSize,
    pub text_findings: Vec<ContrastFinding>,
    pub ui_findings: Vec<UiComponentFinding>,
}

impl ThemeAuditReport {
    pub fn text_failures(&self) -> impl Iterator<Item = &ContrastFinding> {
        self.text_findings
            .iter()
            .filter(|f| f.level == WcagLevel::Fail)
    }

    pub fn ui_failures(&self) -> impl Iterator<Item = &UiComponentFinding> {
        self.ui_findings.iter().filter(|f| !f.passes)
    }

    pub fn is_aaa_compliant(&self) -> bool {
        self.text_findings.iter().all(|f| f.level == WcagLevel::AAA)
    }

    pub fn is_aa_compliant(&self) -> bool {
        self.text_findings
            .iter()
            .all(|f| f.level != WcagLevel::Fail)
            && self.ui_findings.iter().all(|f| f.passes)
    }
}

/// Text-bearing pairs: readable body/label text against every surface it can render on,
/// plus each semantic brand color rendered as text/icon on top of its own tonal container.
type ColorFn = fn(&ColorSystem) -> Color;
const TEXT_PAIRS: &[(&str, ColorFn, ColorFn)] = &[
    ("foreground/background", |c| c.foreground, |c| c.background),
    ("foreground/surface", |c| c.foreground, |c| c.surface),
    (
        "foreground/surface_variant",
        |c| c.foreground,
        |c| c.surface_variant,
    ),
    (
        "foreground/surface_dim",
        |c| c.foreground,
        |c| c.surface_dim,
    ),
    (
        "foreground_secondary/background",
        |c| c.foreground_secondary,
        |c| c.background,
    ),
    (
        "foreground_secondary/surface",
        |c| c.foreground_secondary,
        |c| c.surface,
    ),
    (
        "foreground_tertiary/background",
        |c| c.foreground_tertiary,
        |c| c.background,
    ),
    (
        "foreground_tertiary/surface",
        |c| c.foreground_tertiary,
        |c| c.surface,
    ),
    ("primary/background", |c| c.primary, |c| c.background),
    ("primary/surface", |c| c.primary, |c| c.surface),
    (
        "primary/primary_container",
        |c| c.primary,
        |c| c.primary_container,
    ),
    ("secondary/background", |c| c.secondary, |c| c.background),
    ("secondary/surface", |c| c.secondary, |c| c.surface),
    (
        "secondary/secondary_container",
        |c| c.secondary,
        |c| c.secondary_container,
    ),
    ("accent/background", |c| c.accent, |c| c.background),
    ("accent/surface", |c| c.accent, |c| c.surface),
    ("error/background", |c| c.error, |c| c.background),
    ("error/surface", |c| c.error, |c| c.surface),
    ("error/error_container", |c| c.error, |c| c.error_container),
    ("warning/background", |c| c.warning, |c| c.background),
    ("warning/surface", |c| c.warning, |c| c.surface),
    (
        "warning/warning_container",
        |c| c.warning,
        |c| c.warning_container,
    ),
    ("success/background", |c| c.success, |c| c.background),
    ("success/surface", |c| c.success, |c| c.surface),
    (
        "success/success_container",
        |c| c.success,
        |c| c.success_container,
    ),
    ("info/background", |c| c.info, |c| c.background),
    ("info/surface", |c| c.info, |c| c.surface),
    ("info/info_container", |c| c.info, |c| c.info_container),
];

/// Non-text UI component pairs (borders): `outline` is the functional border/divider
/// color used against these surfaces. `outline_variant` is a decorative-only divider
/// and is intentionally not held to SC 1.4.11 (which applies to components required to
/// understand content, not purely decorative ones).
const UI_PAIRS: &[(&str, ColorFn, ColorFn)] = &[
    ("outline/background", |c| c.outline, |c| c.background),
    ("outline/surface", |c| c.outline, |c| c.surface),
];

/// Audit a single theme's real color tokens for the given text size.
pub fn audit_theme(colors: &ColorSystem, size: TextSize) -> ThemeAuditReport {
    let text_findings = TEXT_PAIRS
        .iter()
        .map(|&(label, fg, bg)| {
            let foreground = fg(colors);
            let background = bg(colors);
            ContrastFinding {
                pair: label,
                foreground_hex: foreground.to_hex(),
                background_hex: background.to_hex(),
                ratio: foreground.contrast_ratio(&background),
                level: wcag_level(&foreground, &background, size),
            }
        })
        .collect();

    let ui_findings = UI_PAIRS
        .iter()
        .map(|&(label, fg, bg)| {
            let foreground = fg(colors);
            let background = bg(colors);
            UiComponentFinding {
                pair: label,
                ratio: foreground.contrast_ratio(&background),
                passes: passes_ui_component_contrast(&foreground, &background),
            }
        })
        .collect();

    ThemeAuditReport {
        theme: colors.theme(),
        size,
        text_findings,
        ui_findings,
    }
}

/// Audit every shipped theme (Light, Dark, OLED, HDR) for the given text size.
pub fn audit_all_themes(size: TextSize) -> Vec<ThemeAuditReport> {
    [
        ThemeName::Light,
        ThemeName::Dark,
        ThemeName::OLED,
        ThemeName::HDR,
    ]
    .iter()
    .map(|&theme| audit_theme(&ColorSystem::from_theme(theme), size))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_covers_every_semantic_pair() {
        let report = audit_theme(&ColorSystem::from_theme(ThemeName::Light), TextSize::Normal);
        assert_eq!(report.text_findings.len(), TEXT_PAIRS.len());
        assert_eq!(report.ui_findings.len(), UI_PAIRS.len());
    }

    #[test]
    fn audit_all_themes_covers_all_four_themes() {
        let reports = audit_all_themes(TextSize::Normal);
        assert_eq!(reports.len(), 4);
        let themes: Vec<_> = reports.iter().map(|r| r.theme).collect();
        assert!(themes.contains(&ThemeName::Light));
        assert!(themes.contains(&ThemeName::Dark));
        assert!(themes.contains(&ThemeName::OLED));
        assert!(themes.contains(&ThemeName::HDR));
    }

    // -- Regression guards: every theme must clear the non-text UI component minimum
    // (3:1) and must never *fail* text contrast outright, at any size. These are the
    // real, currently-shipping color tokens computed via aurora-color's WCAG math --
    // not fixtures, so a future palette change that regresses contrast breaks these.

    #[test]
    fn no_theme_has_ui_component_failures() {
        for report in audit_all_themes(TextSize::Normal) {
            let failures: Vec<_> = report.ui_failures().collect();
            assert!(
                failures.is_empty(),
                "{:?} has UI component contrast failures: {:#?}",
                report.theme,
                failures
            );
        }
    }

    #[test]
    fn no_theme_fails_large_text_contrast_outright() {
        for report in audit_all_themes(TextSize::Large) {
            let failures: Vec<_> = report.text_failures().collect();
            assert!(
                failures.is_empty(),
                "{:?} has large-text contrast failures: {:#?}",
                report.theme,
                failures
            );
        }
    }

    #[test]
    fn no_theme_fails_normal_text_contrast_outright() {
        for report in audit_all_themes(TextSize::Normal) {
            let failures: Vec<_> = report.text_failures().collect();
            assert!(
                failures.is_empty(),
                "{:?} has normal-text contrast failures (below AA): {:#?}",
                report.theme,
                failures
            );
        }
    }

    #[test]
    fn every_theme_is_aa_compliant() {
        for report in audit_all_themes(TextSize::Normal) {
            assert!(
                report.is_aa_compliant(),
                "{:?} is not AA compliant",
                report.theme
            );
        }
    }

    #[test]
    fn every_theme_is_aaa_compliant_for_large_text() {
        for report in audit_all_themes(TextSize::Large) {
            assert!(
                report.is_aaa_compliant(),
                "{:?} is not AAA compliant for large text: {:#?}",
                report.theme,
                report
                    .text_findings
                    .iter()
                    .filter(|f| f.level != WcagLevel::AAA)
                    .collect::<Vec<_>>()
            );
        }
    }

    /// The core reading pairs (body text on its primary surfaces) already reach full
    /// AAA at normal text size in every theme -- lock that in explicitly since it's
    /// the pairing most GNOME apps actually render at length.
    #[test]
    fn core_reading_pairs_are_aaa_at_normal_text_size_in_every_theme() {
        let core_pairs = ["foreground/background", "foreground/surface"];
        for report in audit_all_themes(TextSize::Normal) {
            for finding in report
                .text_findings
                .iter()
                .filter(|f| core_pairs.contains(&f.pair))
            {
                assert_eq!(
                    finding.level,
                    WcagLevel::AAA,
                    "{:?} {} is only {:?} ({}:1)",
                    report.theme,
                    finding.pair,
                    finding.level,
                    finding.ratio
                );
            }
        }
    }

    /// Not every pair reaches full AAA at normal text size yet (tracked honestly in
    /// the project roadmap as in-progress Phase 4 work). This test documents exactly
    /// which ones so a silent regression below AA -- or an untracked improvement to
    /// AAA -- both get caught, without asserting a stronger claim than is true today.
    #[test]
    fn known_normal_text_aa_only_pairs_match_current_palette() {
        use std::collections::BTreeSet;

        let mut aa_only: BTreeSet<(String, &'static str)> = BTreeSet::new();
        for report in audit_all_themes(TextSize::Normal) {
            for finding in &report.text_findings {
                if finding.level == WcagLevel::AA {
                    aa_only.insert((format!("{:?}", report.theme), finding.pair));
                }
            }
        }

        let expected: BTreeSet<(String, &'static str)> = [
            ("Light", "foreground_tertiary/background"),
            ("Light", "foreground_tertiary/surface"),
            ("Light", "warning/background"),
            ("Light", "warning/surface"),
            ("Light", "warning/warning_container"),
            ("Light", "info/background"),
            ("Light", "info/surface"),
            ("Light", "info/info_container"),
            ("Dark", "foreground_tertiary/background"),
            ("Dark", "foreground_tertiary/surface"),
            ("HDR", "foreground_tertiary/background"),
            ("HDR", "foreground_tertiary/surface"),
        ]
        .into_iter()
        .map(|(theme, pair)| (theme.to_string(), pair))
        .collect();

        assert_eq!(
            aa_only, expected,
            "AA-only (not-yet-AAA) normal-text pairs changed -- update this list if the \
             palette moved a pair from AA to AAA, or investigate if one regressed from AAA to AA"
        );
    }
}
