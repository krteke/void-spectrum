//! Facade tests for optional macro exports.

#![cfg(feature = "macros")]

use core::convert::Infallible;

#[cfg(feature = "seed")]
use spectrum_resolver::resolve_theme;
#[cfg(feature = "seed")]
use spectrum_schema::ThemeSpec;
use spectrum_theme::{
    __private::{ThemeValue, TokenSource},
    Color, FontStyle, FontWeight, Length, LengthUnit, LineHeight, Radius, ShadowLayer,
    ThemeBuildError, define_theme_tokens,
};

define_theme_tokens! {
    pub struct AppTheme {
        editor {
            cursor: Color,
        }
        spacing {
            medium: Length,
        }
        radius {
            card: Radius,
        }
        font {
            body: FontWeight,
            style: FontStyle,
        }
        line_height {
            body: LineHeight,
        }
    }
}

define_theme_tokens! {
    struct ComponentStateTheme {
        component StateButtonTokens {
            fg: Color,
            gap: Length,
        }

        states button: StateButtonTokens {
            normal,
            hover extends normal,
            press_down extends hover,
        }
    }
}

define_theme_tokens! {
    struct ShadowTheme {
        shadow {
            card: ShadowLayer,
        }
    }
}

define_theme_tokens! {
    struct LineHeightTheme {
        line_height {
            body: LineHeight,
        }
    }
}

define_theme_tokens! {
    struct FontStyleTheme {
        font {
            style: FontStyle,
        }
    }
}

define_theme_tokens! {
    struct FontWeightTheme {
        font {
            body: FontWeight,
        }
    }
}

define_theme_tokens! {
    struct RadiusTheme {
        radius {
            card: Radius,
        }
    }
}

define_theme_tokens! {
    struct LengthTheme {
        spacing {
            medium: Length,
        }
    }
}

define_theme_tokens! {
    #[derive(Clone)]
    pub struct CloneTheme {
        palette {
            background: Color,
            surface: Color,
        }
        shape {
            gap: Length,
        }
    }
}

define_theme_tokens! {
    #[derive(Clone, Debug, PartialEq)]
    pub struct MultiDeriveTheme {
        surface {
            background: Color,
        }
    }
}

define_theme_tokens! {
    #[derive(Clone)]
    #[derive(Debug)]
    pub struct SplitAttrTheme {
        surface {
            background: Color,
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/theme_tokens.rs"));

struct StaticSource;

impl TokenSource for StaticSource {
    type Error = Infallible;
}

impl ThemeValue<StaticSource> for Color {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        Ok(Color::new(1, 2, 3))
    }
}

impl ThemeValue<StaticSource> for Length {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        Ok(Length::new(9.0, LengthUnit::Px).expect("finite"))
    }
}

impl ThemeValue<StaticSource> for Radius {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        Ok("7px".parse().expect("radius"))
    }
}

impl ThemeValue<StaticSource> for FontWeight {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        Ok(FontWeight::new(700).expect("weight"))
    }
}

impl ThemeValue<StaticSource> for FontStyle {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        Ok(FontStyle::Oblique)
    }
}

impl ThemeValue<StaticSource> for LineHeight {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        Ok("1.25".parse().expect("line height"))
    }
}

impl ThemeValue<StaticSource> for ShadowLayer {
    fn read(_: &StaticSource, _: &str) -> Result<Self, Infallible> {
        let px = |value| Length::new(value, LengthUnit::Px).expect("finite");
        Ok(
            ShadowLayer::new(Color::new(0, 0, 0), px(0.0), px(2.0), px(6.0), px(0.0))
                .expect("shadow"),
        )
    }
}

struct LengthOnlySource;

impl TokenSource for LengthOnlySource {
    type Error = Infallible;
}

impl ThemeValue<LengthOnlySource> for Length {
    fn read(_: &LengthOnlySource, _: &str) -> Result<Self, Infallible> {
        Ok(Length::new(12.0, LengthUnit::Px).expect("finite"))
    }
}

struct RadiusOnlySource;

impl TokenSource for RadiusOnlySource {
    type Error = Infallible;
}

impl ThemeValue<RadiusOnlySource> for Radius {
    fn read(_: &RadiusOnlySource, _: &str) -> Result<Self, Infallible> {
        Ok("10px".parse().expect("radius"))
    }
}

struct FontWeightOnlySource;

impl TokenSource for FontWeightOnlySource {
    type Error = Infallible;
}

impl ThemeValue<FontWeightOnlySource> for FontWeight {
    fn read(_: &FontWeightOnlySource, _: &str) -> Result<Self, Infallible> {
        Ok(FontWeight::new(500).expect("weight"))
    }
}

struct FontStyleOnlySource;

impl TokenSource for FontStyleOnlySource {
    type Error = Infallible;
}

impl ThemeValue<FontStyleOnlySource> for FontStyle {
    fn read(_: &FontStyleOnlySource, _: &str) -> Result<Self, Infallible> {
        Ok(FontStyle::Italic)
    }
}

struct LineHeightOnlySource;

impl TokenSource for LineHeightOnlySource {
    type Error = Infallible;
}

impl ThemeValue<LineHeightOnlySource> for LineHeight {
    fn read(_: &LineHeightOnlySource, _: &str) -> Result<Self, Infallible> {
        Ok("24px".parse().expect("line height"))
    }
}

#[test]
fn builds_from_a_custom_token_source() {
    let theme = AppTheme::try_from_source(&StaticSource).expect("typed theme");
    let file_theme = FileTheme::try_from_source(&StaticSource).expect("file theme");
    assert_eq!(theme.editor.cursor, Color::new(1, 2, 3));
    assert_eq!(theme.spacing.medium.to_string(), "9px");
    assert_eq!(theme.radius.card.to_string(), "7px");
    assert_eq!(theme.font.body.value(), 700);
    assert_eq!(theme.font.style, FontStyle::Oblique);
    assert_eq!(theme.line_height.body.to_string(), "1.25");
    assert_eq!(file_theme.editor.selection.background, Color::new(1, 2, 3));
    assert_eq!(file_theme.spacing.medium.to_string(), "9px");
    assert_eq!(file_theme.radius.card.to_string(), "7px");
    assert_eq!(file_theme.font.body.value(), 700);
    assert_eq!(file_theme.editor.font_weight.value(), 700);
    assert_eq!(file_theme.font.style, FontStyle::Oblique);
    assert_eq!(file_theme.editor.font_style, FontStyle::Oblique);
    assert_eq!(file_theme.line_height.body.to_string(), "1.25");
    assert_eq!(file_theme.editor.line_height.to_string(), "1.25");
    assert_eq!(file_theme.shadow.card.blur().to_string(), "6px");
    let shadow_theme = ShadowTheme::try_from_source(&StaticSource).expect("shadow theme");
    assert_eq!(shadow_theme.shadow.card.blur().to_string(), "6px");

    let length_theme = LengthTheme::try_from_source(&LengthOnlySource).expect("length theme");
    assert_eq!(length_theme.spacing.medium.to_string(), "12px");
    let radius_theme = RadiusTheme::try_from_source(&RadiusOnlySource).expect("radius theme");
    assert_eq!(radius_theme.radius.card.to_string(), "10px");
    let weight_theme =
        FontWeightTheme::try_from_source(&FontWeightOnlySource).expect("weight theme");
    assert_eq!(weight_theme.font.body.value(), 500);
    let style_theme = FontStyleTheme::try_from_source(&FontStyleOnlySource).expect("style theme");
    assert_eq!(style_theme.font.style, FontStyle::Italic);
    let line_height_theme =
        LineHeightTheme::try_from_source(&LineHeightOnlySource).expect("line-height theme");
    assert_eq!(line_height_theme.line_height.body.to_string(), "24px");
}

#[test]
fn user_derives_apply_to_top_level_and_nested_structs() {
    // These must compile to prove Clone is derived on all structs.
    let theme = CloneTheme::try_from_source(&StaticSource).expect("typed theme");
    let _ = theme.clone();

    // Clone is also derived on the generated sub-struct.
    let _ = theme.palette.clone();
    let _ = theme.shape.clone();
}

#[test]
fn combined_derive_macro_inherits_to_all_structs() {
    let theme = MultiDeriveTheme::try_from_source(&StaticSource).expect("typed theme");

    // Clone
    let _ = theme.clone();
    let _ = theme.surface.clone();

    // Debug (compiles-only check: Debug impl exists)
    let _ = format!("{theme:?}");
    let _ = format!("{:?}", theme.surface);

    // PartialEq (compiles-only check)
    let other = theme.clone();
    assert_eq!(theme, other);
}

#[test]
fn split_attributes_are_all_inherited() {
    let theme = SplitAttrTheme::try_from_source(&StaticSource).expect("typed theme");

    // Both #[derive(Clone)] and #[derive(Debug)] must be present.
    let _ = theme.clone();
    let _ = theme.surface.clone();
    let _ = format!("{theme:?}");
    let _ = format!("{:?}", theme.surface);
}

#[test]
fn no_attributes_still_compiles() {
    // The existing AppTheme has no user attributes and must still compile.
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
            )
            .with_length("spacing.medium", "8px".parse().expect("length"))
            .with_radius("radius.card", "6px".parse().expect("radius"))
            .with_font_weight("font.body", "600".parse().expect("weight"))
            .with_font_style("font.style", "normal".parse().expect("style"))
            .with_line_height("line_height.body", "1.75".parse().expect("line height")),
    )
    .expect("resolved");
    let mut theme = AppTheme::try_from_source(&resolved).expect("typed theme");
    theme.reload(&resolved).expect("reload");

    assert_ne!(theme.editor.cursor, Color::new(0, 0, 255));
    assert_eq!(theme.radius.card.to_string(), "6px");
    assert_eq!(theme.font.body.value(), 600);
    assert_eq!(theme.font.style, FontStyle::Normal);
    assert_eq!(theme.line_height.body.to_string(), "1.75");
}

#[cfg(feature = "seed")]
#[test]
fn file_contract_loads_embedded_values_and_supports_seed_override() {
    let blue = FileTheme::try_load().expect("embedded theme");
    let red = FileTheme::try_load_with_seed(Color::new(255, 0, 0)).expect("red theme");

    assert_ne!(blue.editor.cursor, red.editor.cursor);
    assert_eq!(blue.editor.selection.background, Color::new(16, 32, 48));
    assert_eq!(blue.shadow.card.blur().to_string(), "8px");
    assert_eq!(
        blue.editor.selection.foreground,
        blue.editor.selection.background
    );
    assert_eq!(red.editor.selection.background, Color::new(16, 32, 48));
    assert_eq!(red.overlay.scrim, Color::new_rgba(16, 32, 48, 128));
    assert_eq!(blue.spacing.medium.to_string(), "8px");
    assert_eq!(red.spacing.medium.to_string(), "8px");
    assert_eq!(blue.editor.gutter_width.to_string(), "3rem");
    assert_eq!(blue.radius.card.to_string(), "12px");
    assert_eq!(red.radius.card.to_string(), "12px");
    assert_eq!(blue.font.body.value(), 450);
    assert_eq!(red.editor.font_weight.value(), 450);
    assert_eq!(blue.font.style, FontStyle::Italic);
    assert_eq!(red.editor.font_style, FontStyle::Italic);
    assert_eq!(blue.line_height.body.to_string(), "1.5");
    assert_eq!(red.editor.line_height.to_string(), "20px");
}

#[cfg(feature = "seed")]
#[test]
fn updates_all_material_fields_from_one_runtime_seed() {
    let mut theme = FileTheme::try_load().expect("embedded theme");
    let original_cursor = theme.editor.cursor;
    let original_background = theme.editor.background;
    let fixed_selection = theme.editor.selection.background;

    theme
        .try_set_seed(Color::new(255, 0, 0))
        .expect("updated seed");

    assert_ne!(theme.editor.cursor, original_cursor);
    assert_ne!(theme.editor.background, original_background);
    assert_eq!(theme.editor.selection.background, fixed_selection);
}

#[cfg(feature = "seed")]
#[test]
fn component_states_inherit_missing_resolved_theme_values() {
    let resolved = resolve_theme(
        &ThemeSpec::new("Button")
            .with_color("button.normal.fg", "#010203".parse().expect("color"))
            .with_length("button.normal.gap", "4px".parse().expect("length"))
            .with_color("button.hover.fg", "#040506".parse().expect("color")),
    )
    .expect("resolved");

    let theme = ComponentStateTheme::try_from_source(&resolved).expect("typed theme");

    assert_eq!(theme.button.hover.gap.to_string(), "4px");
    assert_eq!(theme.button.press_down.fg, Color::new(4, 5, 6));
    assert_eq!(theme.button.press_down.gap.to_string(), "4px");
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_line_height_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        LineHeightTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "line_height.body"
    ));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_shadow_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        ShadowTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "shadow.card"
    ));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_font_style_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        FontStyleTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "font.style"
    ));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_font_weight_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        FontWeightTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "font.body"
    ));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_radius_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        RadiusTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "radius.card"
    ));
}

#[cfg(feature = "seed")]
#[test]
fn typed_build_reports_missing_length_tokens() {
    let resolved = resolve_theme(&ThemeSpec::new("Empty")).expect("resolved");

    assert!(matches!(
        LengthTheme::try_from_source(&resolved),
        Err(ThemeBuildError::MissingToken { path }) if path == "spacing.medium"
    ));
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
        Err(ThemeBuildError::SeedFeatureDisabled { path }) if path == "editor.background"
    ));
}
