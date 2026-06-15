//! Tests for line-height token resolution.

use spectrum_core::LineHeight;
use spectrum_resolver::{ResolveError, resolve_line_heights};
use spectrum_schema::ThemeSpec;

fn line_height(value: &str) -> LineHeight {
    value.parse().expect("valid line height")
}

#[test]
fn resolves_line_height_literals_and_reference_chains() {
    let spec = ThemeSpec::new("Demo")
        .with_line_height("line.body", "1.5".parse().expect("literal"))
        .with_line_height("line.strong", "{line.body}".parse().expect("reference"))
        .with_line_height("line.heading", "{line.strong}".parse().expect("reference"));

    let values = resolve_line_heights(&spec).expect("resolved");
    assert_eq!(values["line.strong"], line_height("1.5"));
    assert_eq!(values["line.heading"], line_height("1.5"));
}

#[test]
fn reports_missing_line_height_references() {
    let spec = ThemeSpec::new("Demo")
        .with_line_height("line.body", "{line.missing}".parse().expect("reference"));

    assert_eq!(
        resolve_line_heights(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "line.body".to_owned(),
            reference: "line.missing".to_owned(),
        })
    );
}

#[test]
fn reports_closed_line_height_cycles() {
    let spec = ThemeSpec::new("Demo")
        .with_line_height("first", "{second}".parse().expect("reference"))
        .with_line_height("second", "{first}".parse().expect("reference"));

    assert_eq!(
        resolve_line_heights(&spec),
        Err(ResolveError::CircularReference {
            chain: vec!["first".to_owned(), "second".to_owned(), "first".to_owned()],
        })
    );
}
