//! Facade tests for optional macro exports.

#![cfg(feature = "macros")]

use core::convert::Infallible;

#[cfg(feature = "seed")]
use spectrum_resolver::resolve_theme;
#[cfg(feature = "seed")]
use spectrum_schema::ThemeSpec;
use spectrum_theme::{
    __private::TokenSource, Color, ThemeBuildError, define_theme_tokens, include_theme_tokens,
};

define_theme_tokens! {
    pub struct AppTheme {
        editor {
            cursor: Color,
        }
    }
}

include_theme_tokens! {
    pub struct FileTheme;
    source = "data/theme.toml";
    format = toml;
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
    let file_theme = FileTheme::try_from_source(&StaticSource).expect("file theme");
    assert_eq!(theme.editor.cursor, Color::new(1, 2, 3));
    assert_eq!(file_theme.editor.selection.background, Color::new(1, 2, 3));
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
fn file_contract_loads_embedded_values_and_supports_seed_override() {
    let blue = FileTheme::try_load().expect("embedded theme");
    let red = FileTheme::try_load_with_seed(Color::new(255, 0, 0)).expect("red theme");

    assert_ne!(blue.editor.cursor, red.editor.cursor);
    assert_eq!(blue.editor.selection.background, Color::new(16, 32, 48));
    assert_eq!(
        blue.editor.selection.foreground,
        blue.editor.selection.background
    );
    assert_eq!(red.editor.selection.background, Color::new(16, 32, 48));
    assert_eq!(red.overlay.scrim, Color::new_rgba(16, 32, 48, 128));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_contract_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        AppTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "editor.cursor"
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
        Err(ThemeBuildError::MissingSeed { path }) if path == "editor.cursor"
    ));
}

#[cfg(not(feature = "seed"))]
#[test]
fn embedded_material_bindings_report_the_missing_feature() {
    assert!(matches!(
        FileTheme::try_load(),
        Err(ThemeBuildError::SeedFeatureDisabled { path }) if path == "editor.cursor"
    ));
}
