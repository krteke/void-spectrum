//! Tests for radius token resolution.

use spectrum_core::Radius;
use spectrum_resolver::{ResolveError, resolve_radii};
use spectrum_schema::ThemeSpec;

fn radius(value: &str) -> Radius {
    value.parse().expect("valid radius")
}

#[test]
fn resolves_radius_literals_and_reference_chains() {
    let spec = ThemeSpec::new("Demo")
        .with_radius("radius.small", "4px".parse().expect("literal"))
        .with_radius(
            "radius.medium",
            "{radius.small}".parse().expect("reference"),
        )
        .with_radius(
            "radius.large",
            "{radius.medium}".parse().expect("reference"),
        );

    let radii = resolve_radii(&spec).expect("resolved");
    assert_eq!(radii["radius.medium"], radius("4px"));
    assert_eq!(radii["radius.large"], radius("4px"));
}

#[test]
fn reports_missing_radius_references() {
    let spec = ThemeSpec::new("Demo").with_radius(
        "radius.card",
        "{radius.missing}".parse().expect("reference"),
    );

    assert_eq!(
        resolve_radii(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "radius.card".to_owned(),
            reference: "radius.missing".to_owned(),
        })
    );
}

#[test]
fn reports_closed_radius_cycles() {
    let spec = ThemeSpec::new("Demo")
        .with_radius("first", "{second}".parse().expect("reference"))
        .with_radius("second", "{first}".parse().expect("reference"));

    assert_eq!(
        resolve_radii(&spec),
        Err(ResolveError::CircularReference {
            chain: vec!["first".to_owned(), "second".to_owned(), "first".to_owned()],
        })
    );
}
