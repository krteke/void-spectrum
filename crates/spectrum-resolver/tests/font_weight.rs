//! Tests for font-weight token resolution.

use spectrum_core::FontWeight;
use spectrum_resolver::{ResolveError, resolve_font_weights};
use spectrum_schema::ThemeSpec;

fn weight(value: u16) -> FontWeight {
    FontWeight::new(value).expect("valid weight")
}

#[test]
fn resolves_font_weight_literals_and_reference_chains() {
    let spec = ThemeSpec::new("Demo")
        .with_font_weight("font.body", "400".parse().expect("literal"))
        .with_font_weight("font.strong", "{font.body}".parse().expect("reference"))
        .with_font_weight("font.heading", "{font.strong}".parse().expect("reference"));

    let weights = resolve_font_weights(&spec).expect("resolved");
    assert_eq!(weights["font.strong"], weight(400));
    assert_eq!(weights["font.heading"], weight(400));
}

#[test]
fn reports_missing_font_weight_references() {
    let spec = ThemeSpec::new("Demo")
        .with_font_weight("font.body", "{font.missing}".parse().expect("reference"));

    assert_eq!(
        resolve_font_weights(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "font.body".to_owned(),
            reference: "font.missing".to_owned(),
        })
    );
}

#[test]
fn reports_closed_font_weight_cycles() {
    let spec = ThemeSpec::new("Demo")
        .with_font_weight("first", "{second}".parse().expect("reference"))
        .with_font_weight("second", "{first}".parse().expect("reference"));

    assert_eq!(
        resolve_font_weights(&spec),
        Err(ResolveError::CircularReference {
            chain: vec!["first".to_owned(), "second".to_owned(), "first".to_owned()],
        })
    );
}
