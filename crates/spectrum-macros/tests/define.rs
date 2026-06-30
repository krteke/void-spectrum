//! Compile and behavior tests for inline typed-token generation.

use spectrum_macros::define_theme_tokens;

extern crate self as spectrum_theme;

#[doc(hidden)]
pub mod source {
    pub trait TokenSource {
        type Error;

        fn token<T>(&self, path: &str) -> Result<T, Self::Error>
        where
            T: ThemeValue<Self>,
            Self: Sized,
        {
            T::read(self, path)
        }

        fn is_missing(error: &Self::Error) -> bool {
            let _ = error;
            false
        }
    }

    pub trait ThemeValue<S: TokenSource>: Sized {
        fn read(source: &S, path: &str) -> Result<Self, S::Error>;
    }

    pub fn read_inherited<T, S, const N: usize>(source: &S, paths: [&str; N]) -> Result<T, S::Error>
    where
        T: ThemeValue<S>,
        S: TokenSource,
    {
        let mut missing = None;
        for path in paths {
            match source.token::<T>(path) {
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

impl source::TokenSource for PathSource {
    type Error = core::convert::Infallible;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PathError {
    Missing(String),
    Invalid(String),
}

struct InheritedPathSource;

impl source::TokenSource for InheritedPathSource {
    type Error = PathError;

    fn is_missing(error: &Self::Error) -> bool {
        matches!(error, PathError::Missing(_))
    }
}

struct InvalidHoverSource;

impl source::TokenSource for InvalidHoverSource {
    type Error = PathError;

    fn is_missing(error: &Self::Error) -> bool {
        matches!(error, PathError::Missing(_))
    }
}

impl source::ThemeValue<PathSource> for Px {
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
            "primary_button.normal.fg" => 7,
            "primary_button.normal.bg" => 70,
            "primary_button.hover.fg" => 8,
            "primary_button.hover.bg" => 80,
            "secondary_button.normal.fg" => 9,
            "secondary_button.normal.bg" => 90,
            "secondary_button.hover.fg" => 11,
            "secondary_button.hover.bg" => 100,
            "plain_button.fg" => 5,
            "plain_button.bg" => 50,
            "toolbar.button.fg" => 6,
            "toolbar.button.bg" => 60,
            _ => 0,
        }))
    }
}

impl source::ThemeValue<InheritedPathSource> for Px {
    fn read(_: &InheritedPathSource, path: &str) -> Result<Self, PathError> {
        match path {
            "button.normal.fg" => Ok(Px(1)),
            "button.normal.bg" => Ok(Px(10)),
            "button.hover.bg" => Ok(Px(20)),
            "button.press_down.fg" => Ok(Px(3)),
            "button.focus.fg" => Ok(Px(4)),
            "primary_button.normal.fg" => Ok(Px(7)),
            "primary_button.normal.bg" => Ok(Px(70)),
            "primary_button.hover.bg" => Ok(Px(80)),
            "secondary_button.normal.fg" => Ok(Px(9)),
            "secondary_button.normal.bg" => Ok(Px(90)),
            "secondary_button.hover.bg" => Ok(Px(100)),
            _ => Err(PathError::Missing(path.to_owned())),
        }
    }
}

impl source::ThemeValue<InvalidHoverSource> for Px {
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

define_theme_tokens! {
    pub struct StatelessComponentTheme {
        component PlainButtonTokens {
            fg: Px,
            bg: Px,
        }

        plain_button: PlainButtonTokens,
        toolbar {
            button: PlainButtonTokens,
        }
    }
}

define_theme_tokens! {
    pub struct StateAliasTheme {
        component AliasButtonTokens {
            fg: Px,
            bg: Px,
        }

        states primary_button: AliasButtonTokens {
            normal,
            hover extends normal,
        }

        states secondary_button inherit primary_button,
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
fn builds_stateless_component_instances_from_token_sources() {
    let theme = StatelessComponentTheme::try_from_source(&PathSource).expect("stateless theme");

    assert_eq!(theme.plain_button.fg.0, 5);
    assert_eq!(theme.plain_button.bg.0, 50);
    assert_eq!(theme.toolbar.button.fg.0, 6);
    assert_eq!(theme.toolbar.button.bg.0, 60);
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
fn state_set_aliases_copy_component_and_states() {
    let theme = StateAliasTheme::try_from_source(&PathSource).expect("aliased state theme");

    assert_eq!(theme.primary_button.hover.bg.0, 80);
    assert_eq!(theme.secondary_button.normal.fg.0, 9);
    assert_eq!(
        theme
            .secondary_button
            .get(StateAliasThemeSecondaryButtonState::Hover)
            .bg
            .0,
        100
    );
    assert_eq!(
        StateAliasThemeSecondaryButtonState::Hover.parent(),
        Some(StateAliasThemeSecondaryButtonState::Normal)
    );
}

#[test]
fn state_set_aliases_copy_state_inheritance() {
    let theme =
        StateAliasTheme::try_from_source(&InheritedPathSource).expect("aliased state theme");

    assert_eq!(theme.primary_button.hover.fg.0, 7);
    assert_eq!(theme.secondary_button.hover.fg.0, 9);
}

#[test]
fn state_set_inheritance_keeps_non_missing_errors() {
    let error = StatefulTheme::try_from_source(&InvalidHoverSource)
        .expect_err("invalid child value should not fall back");

    assert_eq!(error, PathError::Invalid("button.hover.fg".to_owned()));
}
