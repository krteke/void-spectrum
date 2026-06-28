//! Compile and behavior tests for inline typed-token generation.

use spectrum_macros::define_theme_tokens;

extern crate self as spectrum_theme;

#[doc(hidden)]
pub mod __private {
    pub trait TokenSource {
        type Error;

        fn is_missing(error: &Self::Error) -> bool {
            let _ = error;
            false
        }
    }

    pub trait TokenValue<S: TokenSource>: Sized {
        fn read(source: &S, path: &str) -> Result<Self, S::Error>;
    }

    pub fn read_inherited<T, S, const N: usize>(source: &S, paths: [&str; N]) -> Result<T, S::Error>
    where
        T: TokenValue<S>,
        S: TokenSource,
    {
        let mut missing = None;
        for path in paths {
            match T::read(source, path) {
                Ok(value) => return Ok(value),
                Err(error) if S::is_missing(&error) => {
                    missing.get_or_insert(error);
                }
                Err(error) => return Err(error),
            }
        }
        Err(missing.expect("inherited token lookup has at least one path"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Px(u16);

struct PathSource;

impl __private::TokenSource for PathSource {
    type Error = core::convert::Infallible;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PathError {
    Missing(String),
    Invalid(String),
}

struct InheritedPathSource;

impl __private::TokenSource for InheritedPathSource {
    type Error = PathError;

    fn is_missing(error: &Self::Error) -> bool {
        matches!(error, PathError::Missing(_))
    }
}

struct InvalidHoverSource;

impl __private::TokenSource for InvalidHoverSource {
    type Error = PathError;

    fn is_missing(error: &Self::Error) -> bool {
        matches!(error, PathError::Missing(_))
    }
}

impl __private::TokenValue<PathSource> for Px {
    fn read(_: &PathSource, path: &str) -> Result<Self, core::convert::Infallible> {
        Ok(Px(match path {
            "button.normal.fg" => 1,
            "button.normal.bg" => 10,
            "button.hover.fg" => 2,
            "button.hover.bg" => 20,
            "button.press_down.fg" => 3,
            "button.press_down.bg" => 30,
            "button.focus.fg" => 4,
            "button.focus.bg" => 40,
            _ => 0,
        }))
    }
}

impl __private::TokenValue<InheritedPathSource> for Px {
    fn read(_: &InheritedPathSource, path: &str) -> Result<Self, PathError> {
        match path {
            "button.normal.fg" => Ok(Px(1)),
            "button.normal.bg" => Ok(Px(10)),
            "button.hover.bg" => Ok(Px(20)),
            "button.press_down.fg" => Ok(Px(3)),
            "button.focus.fg" => Ok(Px(4)),
            _ => Err(PathError::Missing(path.to_owned())),
        }
    }
}

impl __private::TokenValue<InvalidHoverSource> for Px {
    fn read(_: &InvalidHoverSource, path: &str) -> Result<Self, PathError> {
        match path {
            "button.normal.fg" => Ok(Px(1)),
            "button.normal.bg" => Ok(Px(10)),
            "button.hover.fg" => Err(PathError::Invalid(path.to_owned())),
            _ => Err(PathError::Missing(path.to_owned())),
        }
    }
}

define_theme_tokens! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct StatefulTheme {
        component ButtonTokens {
            fg: Px,
            bg: Px,
        }

        states button: ButtonTokens {
            normal,
            hover extends normal,
            press_down extends hover,
            focus extends normal,
        }
    }
}

define_theme_tokens! {
    pub struct AppTheme {
        color {
            text {
                primary: Color,
                muted: Color,
            }
        }
        spacing {
            medium: Px,
        }
    }
}

fn primary(theme: &AppTheme) -> Color {
    theme.color.text.primary
}

fn spacing(theme: &AppTheme) -> u16 {
    theme.spacing.medium.0
}

fn foreground(token: &ButtonTokens) -> u16 {
    token.fg.0
}

#[test]
fn generates_nested_typed_fields() {
    let theme = AppTheme {
        color: AppThemeColor {
            text: AppThemeColorText {
                primary: Color,
                muted: Color,
            },
        },
        spacing: AppThemeSpacing { medium: Px(8) },
    };

    let _ = primary(&theme);
    assert_eq!(spacing(&theme), 8);
}

#[test]
fn generates_reusable_component_state_sets() {
    let theme = StatefulTheme {
        button: StatefulThemeButtonStates {
            normal: ButtonTokens {
                fg: Px(1),
                bg: Px(10),
            },
            hover: ButtonTokens {
                fg: Px(2),
                bg: Px(20),
            },
            press_down: ButtonTokens {
                fg: Px(3),
                bg: Px(30),
            },
            focus: ButtonTokens {
                fg: Px(4),
                bg: Px(40),
            },
        },
    };

    assert_eq!(foreground(&theme.button.hover), 2);
    assert_eq!(
        theme.button.get(StatefulThemeButtonState::PressDown).bg.0,
        30
    );
    assert_eq!(
        StatefulThemeButtonState::PressDown.parent(),
        Some(StatefulThemeButtonState::Hover)
    );
    assert_eq!(StatefulThemeButtonState::Normal.parent(), None);
}

#[test]
fn builds_component_state_sets_from_token_sources() {
    let theme = StatefulTheme::try_from_source(&PathSource).expect("stateful theme");

    assert_eq!(theme.button.normal.fg.0, 1);
    assert_eq!(theme.button.hover.bg.0, 20);
    assert_eq!(theme.button.press_down.fg.0, 3);
    assert_eq!(theme.button.focus.bg.0, 40);
}

#[test]
fn state_sets_inherit_missing_values_from_parent_states() {
    let mut theme = StatefulTheme::try_from_source(&InheritedPathSource).expect("stateful theme");

    assert_eq!(theme.button.hover.fg.0, 1);
    assert_eq!(theme.button.press_down.bg.0, 20);
    assert_eq!(theme.button.focus.bg.0, 10);

    theme.reload(&InheritedPathSource).expect("reload");
    assert_eq!(theme.button.press_down.bg.0, 20);
}

#[test]
fn state_set_inheritance_keeps_non_missing_errors() {
    let error = StatefulTheme::try_from_source(&InvalidHoverSource)
        .expect_err("invalid child value should not fall back");

    assert_eq!(error, PathError::Invalid("button.hover.fg".to_owned()));
}
