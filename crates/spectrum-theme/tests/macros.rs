//! Facade tests for optional macro exports.

#![cfg(feature = "macros")]

use core::convert::Infallible;

#[cfg(feature = "toml")]
use spectrum_theme::config::TomlThemeSource;
use spectrum_theme::{
    Color, FontStyle, FontWeight, Length, LengthUnit, LineHeight, Radius, ShadowLayer,
    ThemeBuildError, define_theme_tokens,
    source::{ThemeValue, TokenSource},
};

#[cfg(feature = "toml")]
/// Test-only custom duration token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delay(u16);

#[cfg(feature = "toml")]
/// Test-only custom easing token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Easing {
    /// Linear interpolation.
    Linear,
    /// Ease-out interpolation.
    EaseOut,
}

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
    struct ComponentInstanceTheme {
        component PlainButtonTokens {
            fg: Color,
            gap: Length,
        }

        button: PlainButtonTokens,
        toolbar {
            primary: PlainButtonTokens,
        }
    }
}

#[cfg(feature = "toml")]
define_theme_tokens! {
    struct CustomConfigTheme {
        component MotionTokens {
            delay: Delay,
            easing: Easing,
        }

        motion {
            delay: Delay,
            easing: Easing,
        }

        transition: MotionTokens,

        states transition_state: MotionTokens {
            normal,
            hover extends normal,
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

#[cfg(feature = "toml")]
include!(concat!(env!("OUT_DIR"), "/contract_theme_tokens.rs"));

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

#[cfg(feature = "toml")]
impl ThemeValue<TomlThemeSource> for Delay {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        source
            .token_text(path)?
            .parse()
            .map(Self)
            .map_err(
                |error: core::num::ParseIntError| ThemeBuildError::InvalidTokenValue {
                    path: path.to_owned(),
                    message: error.to_string(),
                },
            )
    }
}

#[cfg(feature = "toml")]
impl ThemeValue<TomlThemeSource> for Easing {
    fn read(source: &TomlThemeSource, path: &str) -> Result<Self, ThemeBuildError> {
        match source.token_text(path)?.as_str() {
            "linear" => Ok(Self::Linear),
            "ease-out" => Ok(Self::EaseOut),
            value => Err(ThemeBuildError::InvalidTokenValue {
                path: path.to_owned(),
                message: format!("unknown easing '{value}'"),
            }),
        }
    }
}

struct MissingSource;

impl TokenSource for MissingSource {
    type Error = ThemeBuildError;

    fn is_missing(error: &Self::Error) -> bool {
        matches!(error, ThemeBuildError::MissingToken { .. })
    }
}

macro_rules! impl_missing_value {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ThemeValue<MissingSource> for $ty {
                fn read(_: &MissingSource, path: &str) -> Result<Self, ThemeBuildError> {
                    Err(ThemeBuildError::MissingToken {
                        path: path.to_owned(),
                    })
                }
            }
        )*
    };
}

impl_missing_value!(
    Color,
    FontStyle,
    FontWeight,
    Length,
    LineHeight,
    Radius,
    ShadowLayer,
);

#[test]
fn builds_from_a_custom_token_source() {
    let theme = AppTheme::try_from_source(&StaticSource).expect("typed theme");
    assert_eq!(theme.editor.cursor, Color::new(1, 2, 3));
    assert_eq!(theme.spacing.medium.to_string(), "9px");
    assert_eq!(theme.radius.card.to_string(), "7px");
    assert_eq!(theme.font.body.value(), 700);
    assert_eq!(theme.font.style, FontStyle::Oblique);
    assert_eq!(theme.line_height.body.to_string(), "1.25");
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

#[cfg(all(feature = "toml", feature = "seed"))]
#[test]
fn builds_material_bindings_from_contract_aware_toml() {
    let source: TomlThemeSource = r##"
seed = "#0000ff"

[meta]
mode = "light"

[editor]
cursor = "{material.primary}"

[spacing]
medium = "8px"

[radius]
card = "6px"

[font]
body = "600"
style = "normal"

[line_height]
body = "1.75"
"##
    .parse()
    .expect("TOML source");

    let mut theme = AppTheme::try_from_source(&source).expect("typed theme");
    theme.reload(&source).expect("reload");

    assert_ne!(theme.editor.cursor, Color::new(0, 0, 255));
    assert_eq!(theme.radius.card.to_string(), "6px");
    assert_eq!(theme.font.body.value(), 600);
    assert_eq!(theme.font.style, FontStyle::Normal);
    assert_eq!(theme.line_height.body.to_string(), "1.75");
}

#[cfg(feature = "toml")]
#[test]
fn external_contract_file_loads_contract_aware_values() {
    let theme = ContractFileTheme::try_load().expect("contract theme");

    assert_eq!(theme.file_button.fg, Color::new(1, 2, 3));
    assert_eq!(theme.file_button.gap.to_string(), "2px");
    assert_eq!(theme.button.normal.fg, Color::new(103, 80, 164));
    assert_eq!(theme.button.hover.fg, Color::new(4, 5, 6));
    assert_eq!(theme.button.hover.gap.to_string(), "4px");
    assert_eq!(theme.button.press_down.fg, Color::new(4, 5, 6));
    assert_eq!(theme.button.press_down.gap.to_string(), "4px");
    assert_eq!(theme.motion.delay, Delay(90));
    assert_eq!(theme.motion.easing, Easing::EaseOut);
}

#[cfg(feature = "toml")]
#[test]
fn component_states_load_from_contract_aware_toml() {
    let source: TomlThemeSource = r##"
seed = "#6750a4"

[meta]
mode = "light"

[button.normal]
fg = "{material.primary}"
gap = "4px"

[button.hover]
fg = "#040506"
"##
    .parse()
    .expect("TOML source");

    let theme = ComponentStateTheme::try_from_source(&source).expect("typed theme");

    assert_eq!(theme.button.normal.fg, Color::new(103, 80, 164));
    assert_eq!(theme.button.hover.gap.to_string(), "4px");
    assert_eq!(theme.button.press_down.fg, Color::new(4, 5, 6));
    assert_eq!(theme.button.press_down.gap.to_string(), "4px");
}

#[cfg(feature = "toml")]
#[test]
fn stateless_component_instances_load_from_contract_aware_toml() {
    let source: TomlThemeSource = r##"
[button]
fg = "#010203"
gap = "4px"

[toolbar.primary]
fg = "#040506"
gap = "8px"
"##
    .parse()
    .expect("TOML source");

    let theme = ComponentInstanceTheme::try_from_source(&source).expect("typed theme");

    assert_eq!(theme.button.fg, Color::new(1, 2, 3));
    assert_eq!(theme.button.gap.to_string(), "4px");
    assert_eq!(theme.toolbar.primary.fg, Color::new(4, 5, 6));
    assert_eq!(theme.toolbar.primary.gap.to_string(), "8px");
}

#[cfg(feature = "toml")]
#[test]
fn custom_values_load_from_contract_aware_toml() {
    let source: TomlThemeSource = r#"
[motion]
delay = 120
easing = "linear"

[transition]
delay = 80
easing = "ease-out"

[transition_state.normal]
delay = 100
easing = "linear"

[transition_state.hover]
easing = "ease-out"
"#
    .parse()
    .expect("TOML source");

    let theme = CustomConfigTheme::try_from_source(&source).expect("typed theme");

    assert_eq!(theme.motion.delay, Delay(120));
    assert_eq!(theme.motion.easing, Easing::Linear);
    assert_eq!(theme.transition.delay, Delay(80));
    assert_eq!(theme.transition.easing, Easing::EaseOut);
    assert_eq!(theme.transition_state.hover.delay, Delay(100));
    assert_eq!(theme.transition_state.hover.easing, Easing::EaseOut);
}

#[test]
fn typed_build_reports_missing_line_height_tokens() {
    assert!(matches!(
        LineHeightTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "line_height.body"
    ));
}

#[test]
fn typed_build_reports_missing_shadow_tokens() {
    assert!(matches!(
        ShadowTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "shadow.card"
    ));
}

#[test]
fn typed_build_reports_missing_font_style_tokens() {
    assert!(matches!(
        FontStyleTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "font.style"
    ));
}

#[test]
fn typed_build_reports_missing_font_weight_tokens() {
    assert!(matches!(
        FontWeightTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "font.body"
    ));
}

#[test]
fn typed_build_reports_missing_radius_tokens() {
    assert!(matches!(
        RadiusTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "radius.card"
    ));
}

#[test]
fn typed_build_reports_missing_length_tokens() {
    assert!(matches!(
        LengthTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "spacing.medium"
    ));
}

#[test]
fn typed_build_reports_missing_contract_tokens() {
    assert!(matches!(
        AppTheme::try_from_source(&MissingSource),
        Err(ThemeBuildError::MissingToken { path }) if path == "editor.cursor"
    ));
}

#[cfg(all(feature = "toml", feature = "seed"))]
#[test]
fn typed_build_requires_seed_for_material_bindings() {
    let source: TomlThemeSource = r#"
[editor]
cursor = "{material.primary}"
"#
    .parse()
    .expect("TOML source");

    assert!(matches!(
        AppTheme::try_from_source(&source),
        Err(ThemeBuildError::MissingSeed { path }) if path == "editor.cursor"
    ));
}

#[cfg(all(feature = "toml", not(feature = "seed")))]
#[test]
fn material_bindings_report_the_missing_seed_feature() {
    assert!(matches!(
        ContractFileTheme::try_load(),
        Err(ThemeBuildError::SeedFeatureDisabled { path }) if path == "button.normal.fg"
    ));
}
