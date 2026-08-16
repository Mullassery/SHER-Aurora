//! SVG Icon Generation - generic SVG-building primitives (viewBox, path
//! elements) for procedurally assembling icon markup.
//!
//! This module is a generic SVG string builder; it does not itself hold any
//! glyph geometry. Aurora's real, hand-authored icon artwork lives in the
//! `aurora-icons` crate (`aurora_icons::icon_svg`).

use crate::icons::{IconContext, IconSize};

/// SVG viewBox dimensions
#[derive(Debug, Clone)]
pub struct ViewBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl ViewBox {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn standard() -> Self {
        Self::new(0, 0, 24, 24)
    }
}

impl std::fmt::Display for ViewBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.x, self.y, self.width, self.height)
    }
}

/// SVG path element
#[derive(Debug, Clone)]
pub struct PathElement {
    d: String,
    stroke: std::option::Option<String>,
    fill: std::option::Option<String>,
    stroke_width: f32,
    stroke_linecap: String,
    stroke_linejoin: String,
}

impl PathElement {
    pub fn new(d: &str) -> Self {
        Self {
            d: d.to_string(),
            stroke: std::option::Option::None,
            fill: std::option::Option::None,
            stroke_width: 1.5,
            stroke_linecap: "round".to_string(),
            stroke_linejoin: "round".to_string(),
        }
    }

    pub fn with_stroke(mut self, color: &str) -> Self {
        self.stroke = std::option::Option::Some(color.to_string());
        self
    }

    pub fn with_fill(mut self, color: &str) -> Self {
        self.fill = std::option::Option::Some(color.to_string());
        self
    }

    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::from("  <path");
        svg.push_str(&format!(" d=\"{}\"", self.d));

        if let Some(ref fill) = self.fill {
            svg.push_str(&format!(" fill=\"{}\"", fill));
        } else {
            svg.push_str(" fill=\"none\"");
        }

        if let Some(ref stroke) = self.stroke {
            svg.push_str(&format!(" stroke=\"{}\"", stroke));
        }

        svg.push_str(&format!(" stroke-width=\"{}\"", self.stroke_width));
        svg.push_str(&format!(" stroke-linecap=\"{}\"", self.stroke_linecap));
        svg.push_str(&format!(" stroke-linejoin=\"{}\"", self.stroke_linejoin));
        svg.push_str(" />\n");
        svg
    }
}

/// SVG circle element
#[derive(Debug, Clone)]
pub struct CircleElement {
    cx: f32,
    cy: f32,
    r: f32,
    fill: std::option::Option<String>,
    stroke: std::option::Option<String>,
    stroke_width: f32,
}

impl CircleElement {
    pub fn new(cx: f32, cy: f32, r: f32) -> Self {
        Self {
            cx,
            cy,
            r,
            fill: std::option::Option::None,
            stroke: std::option::Option::None,
            stroke_width: 1.5,
        }
    }

    pub fn with_fill(mut self, color: &str) -> Self {
        self.fill = std::option::Option::Some(color.to_string());
        self
    }

    pub fn with_stroke(mut self, color: &str) -> Self {
        self.stroke = std::option::Option::Some(color.to_string());
        self
    }

    pub fn to_svg(&self) -> String {
        let mut svg = String::from("  <circle");
        svg.push_str(&format!(" cx=\"{}\"", self.cx));
        svg.push_str(&format!(" cy=\"{}\"", self.cy));
        svg.push_str(&format!(" r=\"{}\"", self.r));

        if let Some(ref fill) = self.fill {
            svg.push_str(&format!(" fill=\"{}\"", fill));
        }
        if let Some(ref stroke) = self.stroke {
            svg.push_str(&format!(" stroke=\"{}\"", stroke));
            svg.push_str(&format!(" stroke-width=\"{}\"", self.stroke_width));
        }

        svg.push_str(" />\n");
        svg
    }
}

/// SVG icon builder
pub struct SvgIconBuilder {
    name: String,
    viewbox: ViewBox,
    paths: Vec<PathElement>,
    circles: Vec<CircleElement>,
    size: IconSize,
    context: IconContext,
}

impl SvgIconBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            viewbox: ViewBox::standard(),
            paths: Vec::new(),
            circles: Vec::new(),
            size: IconSize::Small,
            context: IconContext::Primary,
        }
    }

    pub fn with_size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_context(mut self, context: IconContext) -> Self {
        self.context = context;
        self
    }

    pub fn add_path(mut self, path: PathElement) -> Self {
        self.paths.push(path);
        self
    }

    pub fn add_circle(mut self, circle: CircleElement) -> Self {
        self.circles.push(circle);
        self
    }

    pub fn build(&self) -> String {
        let mut svg = String::new();
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{}\" width=\"{}\" height=\"{}\" class=\"aurora-icon aurora-icon-{}\">\n",
            self.viewbox,
            self.size.pixels(),
            self.size.pixels(),
            self.name
        ));

        // Add all elements
        for circle in &self.circles {
            svg.push_str(&circle.to_svg());
        }

        for path in &self.paths {
            svg.push_str(&path.to_svg());
        }

        svg.push_str("</svg>\n");
        svg
    }
}

/// Generate SVG for a common icon
pub fn generate_icon_svg(
    icon_name: &str,
    size: IconSize,
    context: IconContext,
) -> std::option::Option<String> {
    match icon_name {
        "home" => Some(generate_home_icon(size, context)),
        "save" => Some(generate_save_icon(size, context)),
        "delete" => Some(generate_delete_icon(size, context)),
        "settings" => Some(generate_settings_icon(size, context)),
        "search" => Some(generate_search_icon(size, context)),
        "menu" => Some(generate_menu_icon(size, context)),
        "close" => Some(generate_close_icon(size, context)),
        "check" => Some(generate_check_icon(size, context)),
        "alert" => Some(generate_alert_icon(size, context)),
        "info" => Some(generate_info_icon(size, context)),
        _ => std::option::Option::None,
    }
}

fn generate_home_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("home")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M3 10l9-8 9 8v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-11z")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .add_path(
            PathElement::new("M9 21v-6h6v6")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_save_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("save")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .add_path(
            PathElement::new("M17 21v-8H7v8")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .add_path(
            PathElement::new("M7 3v5h8V3")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_delete_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("delete")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M3 6h18M8 3h8a1 1 0 0 1 1 1v2H7V4a1 1 0 0 1 1-1zM19 9v11a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V9")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .add_path(
            PathElement::new("M10 14v4M14 14v4")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_settings_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("settings")
        .with_size(size)
        .with_context(context)
        .add_circle(CircleElement::new(12.0, 12.0, 3.0).with_stroke(context.color()))
        .add_path(
            PathElement::new("M12 1v6M12 17v6M4.22 4.22l4.24 4.24M15.54 15.54l4.24 4.24M1 12h6M17 12h6M4.22 19.78l4.24-4.24M15.54 8.46l4.24-4.24")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_search_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("search")
        .with_size(size)
        .with_context(context)
        .add_circle(CircleElement::new(11.0, 11.0, 8.0).with_stroke(context.color()))
        .add_path(
            PathElement::new("M21 21l-4.35-4.35")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_menu_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("menu")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M3 6h18M3 12h18M3 18h18")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_close_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("close")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M18 6L6 18M6 6l12 12")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_check_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("check")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M20 6L9 17l-5-5")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

fn generate_alert_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("alert")
        .with_size(size)
        .with_context(context)
        .add_path(
            PathElement::new("M12 2L2 20h20L12 2z")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .add_circle(CircleElement::new(12.0, 16.0, 1.0).with_fill(context.color()))
        .build()
}

fn generate_info_icon(size: IconSize, context: IconContext) -> String {
    SvgIconBuilder::new("info")
        .with_size(size)
        .with_context(context)
        .add_circle(CircleElement::new(12.0, 12.0, 10.0).with_stroke(context.color()))
        .add_circle(CircleElement::new(12.0, 8.0, 1.0).with_fill(context.color()))
        .add_path(
            PathElement::new("M12 12v5")
                .with_stroke(context.color())
                .with_stroke_width(size.stroke_width()),
        )
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewbox_creation() {
        let vb = ViewBox::standard();
        assert_eq!(vb.width, 24);
        assert_eq!(vb.height, 24);
    }

    #[test]
    fn test_viewbox_to_string() {
        let vb = ViewBox::standard();
        assert_eq!(vb.to_string(), "0 0 24 24");
    }

    #[test]
    fn test_path_element() {
        let path = PathElement::new("M10 10 L20 20")
            .with_stroke("#000000")
            .with_stroke_width(2.0);

        let svg = path.to_svg();
        assert!(svg.contains("d=\"M10 10 L20 20\""));
        assert!(svg.contains("stroke=\"#000000\""));
        assert!(svg.contains("stroke-width=\"2\""));
    }

    #[test]
    fn test_circle_element() {
        let circle = CircleElement::new(12.0, 12.0, 10.0).with_fill("#FF0000");

        let svg = circle.to_svg();
        assert!(svg.contains("cx=\"12\""));
        assert!(svg.contains("fill=\"#FF0000\""));
    }

    #[test]
    fn test_svg_icon_builder() {
        let svg = SvgIconBuilder::new("test-icon")
            .with_size(IconSize::Small)
            .add_path(PathElement::new("M0 0 L10 10").with_stroke("#000000"))
            .build();

        assert!(svg.contains("aurora-icon"));
        assert!(svg.contains("test-icon"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_generate_home_icon() {
        let svg = generate_home_icon(IconSize::Small, IconContext::Primary);
        assert!(svg.contains("home"));
        assert!(svg.contains("#003D99")); // Primary color
    }

    #[test]
    fn test_generate_save_icon() {
        let svg = generate_save_icon(IconSize::Medium, IconContext::Success);
        assert!(svg.contains("save"));
        assert!(svg.contains("#004400")); // Success color
    }

    #[test]
    fn test_generate_delete_icon() {
        let svg = generate_delete_icon(IconSize::Large, IconContext::Error);
        assert!(svg.contains("delete"));
        assert!(svg.contains("#990000")); // Error color
    }

    #[test]
    fn test_generate_icon_svg_all() {
        let icons = vec![
            "home", "save", "delete", "settings", "search", "menu", "close", "check", "alert",
            "info",
        ];

        for icon in icons {
            let svg = generate_icon_svg(icon, IconSize::Small, IconContext::Primary);
            assert!(svg.is_some(), "Failed to generate {}", icon);
            assert!(svg.unwrap().contains(icon));
        }
    }

    #[test]
    fn test_generate_icon_svg_unknown() {
        let svg = generate_icon_svg("unknown-icon", IconSize::Small, IconContext::Primary);
        assert!(svg.is_none());
    }

    #[test]
    fn test_icon_different_sizes() {
        for size in [
            IconSize::ExtraSmall,
            IconSize::Small,
            IconSize::Medium,
            IconSize::Large,
            IconSize::ExtraLarge,
        ] {
            let svg = generate_home_icon(size, IconContext::Primary);
            assert!(svg.contains(&format!("width=\"{}\"", size.pixels())));
            assert!(svg.contains(&format!("height=\"{}\"", size.pixels())));
        }
    }

    #[test]
    fn test_icon_different_contexts() {
        let contexts = [
            (IconContext::Primary, "#003D99"),
            (IconContext::Success, "#004400"),
            (IconContext::Error, "#990000"),
            (IconContext::Warning, "#994400"),
        ];

        for (context, color) in contexts {
            let svg = generate_save_icon(IconSize::Small, context);
            assert!(svg.contains(color));
        }
    }
}
