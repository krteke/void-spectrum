//! Public facade for the Void Spectrum typed theme-token engine.

pub use spectrum_core::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer,
};

#[cfg(feature = "iced")]
pub use spectrum_iced as iced;
#[cfg(feature = "macros")]
pub use spectrum_macros::define_theme_tokens;
#[cfg(feature = "ratatui")]
pub use spectrum_ratatui as ratatui;

/// Errors produced while constructing a generated typed theme.
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

#[doc(hidden)]
pub mod __private {
    use super::{
        Color, FontStyle, FontWeight, Length, LineHeight, Radius, ShadowLayer, ThemeBuildError,
    };

    pub use spectrum_palette::{MaterialColor, MaterialColors};
    pub use spectrum_resolver::{ColorBinding, ResolvedTheme};
    pub use spectrum_schema::{ThemeMeta, ThemeMode};

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

    #[cfg(feature = "seed")]
    pub fn material_colors(
        seed: Color,
        mode: ThemeMode,
        _: &str,
    ) -> Result<MaterialColors, ThemeBuildError> {
        Ok(spectrum_palette::material_colors(seed, mode))
    }

    #[cfg(not(feature = "seed"))]
    pub fn material_colors(
        _: Color,
        _: ThemeMode,
        path: &str,
    ) -> Result<MaterialColors, ThemeBuildError> {
        Err(ThemeBuildError::SeedFeatureDisabled {
            path: path.to_owned(),
        })
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

        fn is_missing(error: &Self::Error) -> bool {
            matches!(error, ThemeBuildError::MissingToken { .. })
        }
    }

    impl ThemeValue<ResolvedTheme> for Color {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_color(source, source.seed, path)
        }
    }

    impl ThemeValue<ResolvedTheme> for Length {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_length(source, path)
        }
    }

    impl ThemeValue<ResolvedTheme> for Radius {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_radius(source, path)
        }
    }

    impl ThemeValue<ResolvedTheme> for FontWeight {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_font_weight(source, path)
        }
    }

    impl ThemeValue<ResolvedTheme> for FontStyle {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_font_style(source, path)
        }
    }

    impl ThemeValue<ResolvedTheme> for LineHeight {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_line_height(source, path)
        }
    }

    impl ThemeValue<ResolvedTheme> for ShadowLayer {
        fn read(source: &ResolvedTheme, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_shadow(source, path)
        }
    }

    impl TokenSource for SeededTheme<'_> {
        type Error = ThemeBuildError;

        fn is_missing(error: &Self::Error) -> bool {
            matches!(error, ThemeBuildError::MissingToken { .. })
        }
    }

    impl ThemeValue<SeededTheme<'_>> for Color {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_color(source.theme, Some(source.seed), path)
        }
    }

    impl ThemeValue<SeededTheme<'_>> for Length {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_length(source.theme, path)
        }
    }

    impl ThemeValue<SeededTheme<'_>> for Radius {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_radius(source.theme, path)
        }
    }

    impl ThemeValue<SeededTheme<'_>> for FontWeight {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_font_weight(source.theme, path)
        }
    }

    impl ThemeValue<SeededTheme<'_>> for FontStyle {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_font_style(source.theme, path)
        }
    }

    impl ThemeValue<SeededTheme<'_>> for LineHeight {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_line_height(source.theme, path)
        }
    }

    impl ThemeValue<SeededTheme<'_>> for ShadowLayer {
        fn read(source: &SeededTheme<'_>, path: &str) -> Result<Self, ThemeBuildError> {
            resolve_shadow(source.theme, path)
        }
    }

    fn resolve_shadow(theme: &ResolvedTheme, path: &str) -> Result<ShadowLayer, ThemeBuildError> {
        theme
            .shadows
            .iter()
            .find_map(|(token, layer)| (token == path).then_some(*layer))
            .ok_or_else(|| ThemeBuildError::MissingToken {
                path: path.to_owned(),
            })
    }

    fn resolve_line_height(
        theme: &ResolvedTheme,
        path: &str,
    ) -> Result<LineHeight, ThemeBuildError> {
        theme
            .line_heights
            .get(path)
            .copied()
            .ok_or_else(|| ThemeBuildError::MissingToken {
                path: path.to_owned(),
            })
    }

    fn resolve_font_style(theme: &ResolvedTheme, path: &str) -> Result<FontStyle, ThemeBuildError> {
        theme
            .font_styles
            .get(path)
            .copied()
            .ok_or_else(|| ThemeBuildError::MissingToken {
                path: path.to_owned(),
            })
    }

    fn resolve_font_weight(
        theme: &ResolvedTheme,
        path: &str,
    ) -> Result<FontWeight, ThemeBuildError> {
        theme
            .font_weights
            .get(path)
            .copied()
            .ok_or_else(|| ThemeBuildError::MissingToken {
                path: path.to_owned(),
            })
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
