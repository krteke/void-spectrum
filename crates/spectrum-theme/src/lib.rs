//! Public facade for the Void Spectrum typed theme-token engine.

pub use spectrum_core::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer,
};

#[cfg(feature = "macros")]
pub use spectrum_macros::{define_theme_tokens, include_theme_tokens};

/// Errors produced while constructing a generated typed theme.
#[cfg(feature = "macros")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ThemeBuildError {
    /// The generated contract requires a token absent from the source.
    #[error("missing token '{path}'")]
    MissingToken {
        /// Missing token path.
        path: String,
    },
    /// A Material binding was loaded without a Seed.
    #[error("Material color token '{path}' requires a Seed")]
    MissingSeed {
        /// Material-bound token path.
        path: String,
    },
    /// A Material binding was loaded without the `seed` feature.
    #[error("Material color token '{path}' requires the 'seed' feature")]
    SeedFeatureDisabled {
        /// Material-bound token path.
        path: String,
    },
}

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod __private {
    use super::{Color, Length, Radius, ThemeBuildError};

    pub use spectrum_palette::MaterialColor;
    pub use spectrum_resolver::{ColorBinding, ResolvedTheme};
    pub use spectrum_schema::{ThemeMeta, ThemeMode};

    pub trait TokenSource {
        type Error;
    }

    pub trait ColorSource: TokenSource {
        fn color(&self, path: &str) -> Result<Color, Self::Error>;
    }

    pub trait LengthSource: TokenSource {
        fn length(&self, path: &str) -> Result<Length, Self::Error>;
    }

    pub trait RadiusSource: TokenSource {
        fn radius(&self, path: &str) -> Result<Radius, Self::Error>;
    }

    pub trait TokenValue<S: TokenSource>: Sized {
        fn read(source: &S, path: &str) -> Result<Self, S::Error>;
    }

    impl<S: ColorSource> TokenValue<S> for Color {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.color(path)
        }
    }

    impl<S: LengthSource> TokenValue<S> for Length {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.length(path)
        }
    }

    impl<S: RadiusSource> TokenValue<S> for Radius {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.radius(path)
        }
    }

    pub struct SeededTheme<'a> {
        theme: &'a ResolvedTheme,
        seed: Color,
    }

    impl<'a> SeededTheme<'a> {
        #[must_use]
        pub const fn new(theme: &'a ResolvedTheme, seed: Color) -> Self {
            Self { theme, seed }
        }
    }

    impl TokenSource for ResolvedTheme {
        type Error = ThemeBuildError;
    }

    impl ColorSource for ResolvedTheme {
        fn color(&self, path: &str) -> Result<Color, Self::Error> {
            resolve_color(self, self.seed, path)
        }
    }

    impl LengthSource for ResolvedTheme {
        fn length(&self, path: &str) -> Result<Length, Self::Error> {
            resolve_length(self, path)
        }
    }

    impl RadiusSource for ResolvedTheme {
        fn radius(&self, path: &str) -> Result<Radius, Self::Error> {
            resolve_radius(self, path)
        }
    }

    impl TokenSource for SeededTheme<'_> {
        type Error = ThemeBuildError;
    }

    impl ColorSource for SeededTheme<'_> {
        fn color(&self, path: &str) -> Result<Color, Self::Error> {
            resolve_color(self.theme, Some(self.seed), path)
        }
    }

    impl LengthSource for SeededTheme<'_> {
        fn length(&self, path: &str) -> Result<Length, Self::Error> {
            resolve_length(self.theme, path)
        }
    }

    impl RadiusSource for SeededTheme<'_> {
        fn radius(&self, path: &str) -> Result<Radius, Self::Error> {
            resolve_radius(self.theme, path)
        }
    }

    fn resolve_radius(theme: &ResolvedTheme, path: &str) -> Result<Radius, ThemeBuildError> {
        theme
            .radii
            .get(path)
            .copied()
            .ok_or_else(|| ThemeBuildError::MissingToken {
                path: path.to_owned(),
            })
    }

    fn resolve_length(theme: &ResolvedTheme, path: &str) -> Result<Length, ThemeBuildError> {
        theme
            .lengths
            .get(path)
            .copied()
            .ok_or_else(|| ThemeBuildError::MissingToken {
                path: path.to_owned(),
            })
    }

    fn resolve_color(
        theme: &ResolvedTheme,
        seed: Option<Color>,
        path: &str,
    ) -> Result<Color, ThemeBuildError> {
        let binding =
            theme
                .colors
                .get(path)
                .copied()
                .ok_or_else(|| ThemeBuildError::MissingToken {
                    path: path.to_owned(),
                })?;
        match binding {
            ColorBinding::Color(color) => Ok(color),
            ColorBinding::Material(role) => resolve_material(seed, theme.meta.mode, role, path),
        }
    }

    #[cfg(feature = "seed")]
    fn resolve_material(
        seed: Option<Color>,
        mode: ThemeMode,
        role: MaterialColor,
        path: &str,
    ) -> Result<Color, ThemeBuildError> {
        let seed = seed.ok_or_else(|| ThemeBuildError::MissingSeed {
            path: path.to_owned(),
        })?;
        Ok(spectrum_palette::material_colors(seed, mode).resolve(role))
    }

    #[cfg(not(feature = "seed"))]
    fn resolve_material(
        _: Option<Color>,
        _: ThemeMode,
        _: MaterialColor,
        path: &str,
    ) -> Result<Color, ThemeBuildError> {
        Err(ThemeBuildError::SeedFeatureDisabled {
            path: path.to_owned(),
        })
    }
}
