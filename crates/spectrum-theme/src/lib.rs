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

    pub trait ColorSource: TokenSource {
        fn color(&self, path: &str) -> Result<Color, Self::Error>;
    }

    pub trait LengthSource: TokenSource {
        fn length(&self, path: &str) -> Result<Length, Self::Error>;
    }

    pub trait RadiusSource: TokenSource {
        fn radius(&self, path: &str) -> Result<Radius, Self::Error>;
    }

    pub trait FontWeightSource: TokenSource {
        fn font_weight(&self, path: &str) -> Result<FontWeight, Self::Error>;
    }

    pub trait FontStyleSource: TokenSource {
        fn font_style(&self, path: &str) -> Result<FontStyle, Self::Error>;
    }

    pub trait LineHeightSource: TokenSource {
        fn line_height(&self, path: &str) -> Result<LineHeight, Self::Error>;
    }

    pub trait ShadowSource: TokenSource {
        fn shadow(&self, path: &str) -> Result<ShadowLayer, Self::Error>;
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

    impl<S: FontWeightSource> TokenValue<S> for FontWeight {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.font_weight(path)
        }
    }

    impl<S: FontStyleSource> TokenValue<S> for FontStyle {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.font_style(path)
        }
    }

    impl<S: LineHeightSource> TokenValue<S> for LineHeight {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.line_height(path)
        }
    }

    impl<S: ShadowSource> TokenValue<S> for ShadowLayer {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.shadow(path)
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

    impl FontWeightSource for ResolvedTheme {
        fn font_weight(&self, path: &str) -> Result<FontWeight, Self::Error> {
            resolve_font_weight(self, path)
        }
    }

    impl FontStyleSource for ResolvedTheme {
        fn font_style(&self, path: &str) -> Result<FontStyle, Self::Error> {
            resolve_font_style(self, path)
        }
    }

    impl LineHeightSource for ResolvedTheme {
        fn line_height(&self, path: &str) -> Result<LineHeight, Self::Error> {
            resolve_line_height(self, path)
        }
    }

    impl ShadowSource for ResolvedTheme {
        fn shadow(&self, path: &str) -> Result<ShadowLayer, Self::Error> {
            resolve_shadow(self, path)
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

    impl FontWeightSource for SeededTheme<'_> {
        fn font_weight(&self, path: &str) -> Result<FontWeight, Self::Error> {
            resolve_font_weight(self.theme, path)
        }
    }

    impl FontStyleSource for SeededTheme<'_> {
        fn font_style(&self, path: &str) -> Result<FontStyle, Self::Error> {
            resolve_font_style(self.theme, path)
        }
    }

    impl LineHeightSource for SeededTheme<'_> {
        fn line_height(&self, path: &str) -> Result<LineHeight, Self::Error> {
            resolve_line_height(self.theme, path)
        }
    }

    impl ShadowSource for SeededTheme<'_> {
        fn shadow(&self, path: &str) -> Result<ShadowLayer, Self::Error> {
            resolve_shadow(self.theme, path)
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
