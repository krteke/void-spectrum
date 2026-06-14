//! Tests for the initial color resolution pass.

use spectrum_core::Color;
use spectrum_palette::{MaterialColor, material_colors};
use spectrum_resolver::{ColorBinding, ResolveError, resolve_colors};
use spectrum_schema::{ColorValue, ThemeSpec};

#[test]
fn resolves_direct_color_values() {
    let spec = ThemeSpec::new("Demo").with_color(
        "accent.primary",
        ColorValue::Literal(Color::new(80, 120, 200)),
    );

    assert_eq!(
        resolve_colors(&spec).expect("resolved")["accent.primary"],
        ColorBinding::Color(Color::new(80, 120, 200))
    );
}

#[test]
fn resolves_references_to_direct_colors() {
    let spec = ThemeSpec::new("Demo")
        .with_color("accent.primary", "#5078c8".parse().expect("literal"))
        .with_color(
            "border.focused",
            "{accent.primary}".parse().expect("reference"),
        );

    let colors = resolve_colors(&spec).expect("resolved");
    assert_eq!(colors["border.focused"], colors["accent.primary"]);
}

#[test]
fn reports_missing_references_with_both_paths() {
    let spec = ThemeSpec::new("Demo").with_color(
        "border.focused",
        "{accent.missing}".parse().expect("reference"),
    );

    assert_eq!(
        resolve_colors(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "border.focused".to_owned(),
            reference: "accent.missing".to_owned(),
        })
    );
}

#[test]
fn resolves_reference_chains() {
    let spec = ThemeSpec::new("Demo")
        .with_color("first", "{second}".parse().expect("reference"))
        .with_color("second", "{third}".parse().expect("reference"))
        .with_color("third", "#5078c8".parse().expect("literal"));

    let colors = resolve_colors(&spec).expect("resolved");
    assert_eq!(
        colors["first"],
        ColorBinding::Color(Color::new(80, 120, 200))
    );
    assert_eq!(colors["second"], colors["third"]);
}

#[test]
fn reports_closed_cycle_chain() {
    let spec = ThemeSpec::new("Demo")
        .with_color("first", "{second}".parse().expect("reference"))
        .with_color("second", "{third}".parse().expect("reference"))
        .with_color("third", "{first}".parse().expect("reference"));

    assert_eq!(
        resolve_colors(&spec),
        Err(ResolveError::CircularReference {
            chain: vec![
                "first".to_owned(),
                "second".to_owned(),
                "third".to_owned(),
                "first".to_owned(),
            ],
        })
    );
}

#[test]
fn preserves_material_sources_without_resolving_a_seed() {
    let spec = ThemeSpec::new("Dynamic").with_color(
        "accent.primary",
        "{material.primary}".parse().expect("material reference"),
    );

    assert_eq!(
        resolve_colors(&spec).expect("resolved")["accent.primary"],
        ColorBinding::Material(MaterialColor::Primary)
    );
}

#[test]
fn preserves_extended_material_roles() {
    let spec = ThemeSpec::new("Dynamic").with_color(
        "panel.background",
        "{material.secondary_container}"
            .parse()
            .expect("material reference"),
    );

    assert_eq!(
        resolve_colors(&spec).expect("resolved")["panel.background"],
        ColorBinding::Material(MaterialColor::SecondaryContainer)
    );
}

#[test]
fn one_binding_resolves_against_multiple_seeds() {
    let spec = ThemeSpec::new("Dynamic").with_color(
        "accent.primary",
        "{material.primary}".parse().expect("material reference"),
    );
    let binding = resolve_colors(&spec).expect("resolved")["accent.primary"];
    let blue = material_colors(Color::new(0, 0, 255), spec.meta.mode);
    let red = material_colors(Color::new(255, 0, 0), spec.meta.mode);

    assert_ne!(binding.resolve(blue), binding.resolve(red));
}
