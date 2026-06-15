//! Tests for the resolved theme output.

use spectrum_core::{Color, FontStyle, FontWeight, Length, LengthUnit, LineHeight, Radius};
use spectrum_palette::MaterialColor;
use spectrum_resolver::{ColorBinding, ResolveError, resolve_theme};
use spectrum_schema::LineHeightValue;
use spectrum_schema::{
    FontStyleValue, FontWeightValue, LengthValue, RadiusValue, ThemeMode, ThemeSpec,
};

#[test]
fn resolves_an_owned_theme_output() {
    let gap = Length::new(8.0, LengthUnit::Px).expect("finite");
    let radius = "6px".parse::<Radius>().expect("radius");
    let weight = FontWeight::new(550).expect("weight");
    let mut spec = ThemeSpec::new("Demo")
        .with_seed(Color::new(10, 20, 30))
        .with_color("accent", "#5078c8".parse().expect("literal"))
        .with_color("focus", "{accent}".parse().expect("reference"))
        .with_length("spacing.medium", LengthValue::Literal(gap))
        .with_radius("radius.card", RadiusValue::Literal(radius))
        .with_font_weight("font.body", FontWeightValue::Literal(weight))
        .with_font_style("font.style", FontStyleValue::Literal(FontStyle::Italic))
        .with_line_height(
            "line_height.body",
            LineHeightValue::Literal(LineHeight::multiplier(1.5).expect("line height")),
        );
    spec.meta.mode = ThemeMode::Light;

    let theme = resolve_theme(&spec).expect("resolved theme");

    assert_eq!(theme.meta.name, "Demo");
    assert_eq!(theme.meta.mode, ThemeMode::Light);
    assert_eq!(theme.seed, Some(Color::new(10, 20, 30)));
    assert_eq!(
        theme.colors["focus"],
        ColorBinding::Color(Color::new(80, 120, 200))
    );
    assert_eq!(theme.lengths["spacing.medium"], gap);
    assert_eq!(theme.radii["radius.card"], radius);
    assert_eq!(theme.font_weights["font.body"], weight);
    assert_eq!(theme.font_styles["font.style"], FontStyle::Italic);
    assert_eq!(
        theme.line_heights["line_height.body"],
        LineHeight::multiplier(1.5).expect("line height")
    );
}

#[test]
fn output_does_not_borrow_the_specification() {
    let theme = {
        let spec =
            ThemeSpec::new("Temporary").with_color("accent", "#5078c8".parse().expect("literal"));
        resolve_theme(&spec).expect("resolved theme")
    };

    assert_eq!(theme.meta.name, "Temporary");
}

#[test]
fn propagates_color_resolution_errors() {
    let spec =
        ThemeSpec::new("Broken").with_color("focus", "{missing}".parse().expect("reference"));

    assert_eq!(
        resolve_theme(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "focus".to_owned(),
            reference: "missing".to_owned(),
        })
    );
}

#[test]
fn binds_material_sources_to_configured_tokens() {
    let mut spec = ThemeSpec::new("Seeded")
        .with_seed(Color::new(0, 0, 255))
        .with_color(
            "button.background",
            "{material.primary}".parse().expect("material reference"),
        )
        .with_color(
            "button.hover",
            "{button.background}".parse().expect("token reference"),
        );
    spec.meta.mode = ThemeMode::Light;

    let theme = resolve_theme(&spec).expect("resolved theme");

    assert_eq!(
        theme.colors["button.background"],
        ColorBinding::Material(MaterialColor::Primary)
    );
    assert_eq!(
        theme.colors["button.hover"],
        theme.colors["button.background"]
    );
}

#[test]
fn rejects_unknown_material_roles() {
    let spec = ThemeSpec::new("Seeded")
        .with_seed(Color::new(0, 0, 255))
        .with_color(
            "button.background",
            "{material.unknown}".parse().expect("material reference"),
        );

    assert_eq!(
        resolve_theme(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "button.background".to_owned(),
            reference: "material.unknown".to_owned(),
        })
    );
}
