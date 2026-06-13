//! Tests for the resolved theme output.

use spectrum_core::Color;
use spectrum_resolver::{ResolveError, resolve_theme};
use spectrum_schema::{ThemeMode, ThemeSpec};

#[test]
fn resolves_an_owned_theme_output() {
    let mut spec = ThemeSpec::new("Demo")
        .with_seed(Color::new(10, 20, 30))
        .with_color("accent", "#5078c8".parse().expect("literal"))
        .with_color("focus", "{accent}".parse().expect("reference"));
    spec.meta.mode = ThemeMode::Light;

    let theme = resolve_theme(&spec).expect("resolved theme");

    assert_eq!(theme.meta.name, "Demo");
    assert_eq!(theme.meta.mode, ThemeMode::Light);
    assert_eq!(theme.seed, Some(Color::new(10, 20, 30)));
    assert_eq!(theme.colors["focus"], Color::new(80, 120, 200));
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
