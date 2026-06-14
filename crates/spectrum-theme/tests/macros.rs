//! Facade tests for optional macro exports.

#![cfg(feature = "macros")]

use core::convert::Infallible;

#[cfg(feature = "seed")]
use spectrum_resolver::resolve_theme;
#[cfg(feature = "seed")]
use spectrum_schema::ThemeSpec;
#[cfg(feature = "seed")]
use spectrum_theme::__private::BuildError;
use spectrum_theme::{__private::TokenSource, Color, define_theme_tokens};

define_theme_tokens! {
    pub struct AppTheme {
        editor {
            cursor: Color,
        }
    }
}

struct StaticSource;

impl TokenSource for StaticSource {
    type Error = Infallible;

    fn color(&self, _: &str) -> Result<Color, Self::Error> {
        Ok(Color::new(1, 2, 3))
    }
}

#[test]
fn builds_from_a_custom_token_source() {
    let theme = AppTheme::try_from_source(&StaticSource).expect("typed theme");
    assert_eq!(theme.editor.cursor, Color::new(1, 2, 3));
}

#[cfg(feature = "seed")]
#[test]
fn builds_material_bindings_from_resolved_themes() {
    let resolved = resolve_theme(
        &ThemeSpec::new("Editor")
            .with_seed(Color::new(0, 0, 255))
            .with_color(
                "editor.cursor",
                "{material.primary}".parse().expect("Material reference"),
            ),
    )
    .expect("resolved");
    let theme = AppTheme::try_from_source(&resolved).expect("typed theme");

    assert_ne!(theme.editor.cursor, Color::new(0, 0, 255));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_contract_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        AppTheme::try_from_source(&resolved),
        Err(BuildError::MissingToken { path }) if path == "editor.cursor"
    ));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_requires_seed_for_material_bindings() {
    let resolved = resolve_theme(&ThemeSpec::new("Editor").with_color(
        "editor.cursor",
        "{material.primary}".parse().expect("Material reference"),
    ))
    .expect("resolved");

    assert!(matches!(
        AppTheme::try_from_source(&resolved),
        Err(BuildError::MissingSeed { path }) if path == "editor.cursor"
    ));
}
