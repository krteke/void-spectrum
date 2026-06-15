//! Tests for font-style token resolution.

use spectrum_core::FontStyle;
use spectrum_resolver::{ResolveError, resolve_font_styles};
use spectrum_schema::ThemeSpec;

#[test]
fn resolves_font_style_literals_and_reference_chains() {
    let spec = ThemeSpec::new("Demo")
        .with_font_style("font.body", "normal".parse().expect("literal"))
        .with_font_style("font.emphasis", "{font.body}".parse().expect("reference"))
        .with_font_style(
            "font.heading",
            "{font.emphasis}".parse().expect("reference"),
        );

    let styles = resolve_font_styles(&spec).expect("resolved");
    assert_eq!(styles["font.emphasis"], FontStyle::Normal);
    assert_eq!(styles["font.heading"], FontStyle::Normal);
}

#[test]
fn reports_missing_font_style_references() {
    let spec = ThemeSpec::new("Demo")
        .with_font_style("font.body", "{font.missing}".parse().expect("reference"));

    assert_eq!(
        resolve_font_styles(&spec),
        Err(ResolveError::UnresolvedReference {
            token: "font.body".to_owned(),
            reference: "font.missing".to_owned(),
        })
    );
}

#[test]
fn reports_closed_font_style_cycles() {
    let spec = ThemeSpec::new("Demo")
        .with_font_style("first", "{second}".parse().expect("reference"))
        .with_font_style("second", "{first}".parse().expect("reference"));

    assert_eq!(
        resolve_font_styles(&spec),
        Err(ResolveError::CircularReference {
            chain: vec!["first".to_owned(), "second".to_owned(), "first".to_owned()],
        })
    );
}
