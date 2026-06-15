//! Tests for length token resolution.

use spectrum_core::{Length, LengthUnit};
use spectrum_resolver::{ResolveError, resolve_lengths};
use spectrum_schema::{LengthValue, ThemeSpec};

fn px(value: f32) -> Length {
    Length::new(value, LengthUnit::Px).expect("finite")
}

#[test]
fn resolves_length_literals_and_reference_chains() {
    let spec = ThemeSpec::new("Demo")
        .with_length("spacing.small", LengthValue::Literal(px(4.0)))
        .with_length(
            "spacing.medium",
            "{spacing.small}".parse().expect("reference"),
        )
        .with_length(
            "spacing.large",
            "{spacing.medium}".parse().expect("reference"),
        );

    let lengths = resolve_lengths(&spec).expect("resolved");
    assert_eq!(lengths["spacing.medium"], px(4.0));
    assert_eq!(lengths["spacing.large"], px(4.0));
}

#[test]
fn reports_missing_length_references() {
    let spec = ThemeSpec::new("Demo").with_length(
        "spacing.medium",
        "{spacing.missing}".parse().expect("reference"),
    );

    assert_eq!(
        resolve_lengths(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "spacing.medium".to_owned(),
            reference: "spacing.missing".to_owned(),
        })
    );
}

#[test]
fn reports_closed_length_cycles() {
    let spec = ThemeSpec::new("Demo")
        .with_length("first", "{second}".parse().expect("reference"))
        .with_length("second", "{first}".parse().expect("reference"));

    assert_eq!(
        resolve_lengths(&spec),
        Err(ResolveError::CircularReference {
            chain: vec!["first".to_owned(), "second".to_owned(), "first".to_owned()],
        })
    );
}
