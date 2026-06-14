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

#[derive(Clone, Copy)]
struct Color;

#[derive(Clone, Copy)]
struct Px(u16);

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

#[test]
fn generates_nested_typed_fields() {
    let theme = AppTheme {
        color: AppThemeColorTokens {
            text: AppThemeColorTextTokens {
                primary: Color,
                muted: Color,
            },
        },
        spacing: AppThemeSpacingTokens { medium: Px(8) },
    };

    let _ = primary(&theme);
    assert_eq!(spacing(&theme), 8);
}
