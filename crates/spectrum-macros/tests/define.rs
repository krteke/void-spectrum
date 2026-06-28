//! Compile and behavior tests for inline typed-token generation.

use spectrum_macros::define_theme_tokens;

extern crate self as spectrum_theme;

#[doc(hidden)]
pub mod __private {
    pub trait TokenSource {
        type Error;
    }

    pub trait TokenValue<S: TokenSource>: Sized {
        fn read(source: &S, path: &str) -> Result<Self, S::Error>;
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
