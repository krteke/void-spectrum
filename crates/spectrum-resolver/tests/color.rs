//! Tests for the initial color resolution pass.

use spectrum_core::Color;
use spectrum_resolver::{ResolveError, resolve_colors};
use spectrum_schema::{ColorValue, ThemeSpec};

#[test]
fn resolves_direct_color_values() {
    let spec = ThemeSpec::new("Demo").with_color(
        "accent.primary",
        ColorValue::Literal(Color::new(80, 120, 200)),
    );

    assert_eq!(
        resolve_colors(&spec).expect("resolved")["accent.primary"],
        Color::new(80, 120, 200)
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
fn leaves_reference_chains_for_the_recursive_resolver() {
    let spec = ThemeSpec::new("Demo")
        .with_color("first", "{second}".parse().expect("reference"))
        .with_color("second", "{third}".parse().expect("reference"))
        .with_color("third", "#5078c8".parse().expect("literal"));

    assert!(matches!(
        resolve_colors(&spec),
        Err(ResolveError::UnresolvedReference { .. })
    ));
}
