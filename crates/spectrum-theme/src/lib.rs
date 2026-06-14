//! Public facade for the Void Spectrum typed theme-token engine.

pub use spectrum_core::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer,
};

#[cfg(feature = "macros")]
pub use spectrum_macros::define_theme_tokens;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod __private {
    use spectrum_core::Color;
    #[cfg(feature = "seed")]
    use spectrum_resolver::{ColorBinding, ResolvedTheme};

    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    pub enum BuildError {
        #[error("missing color token '{path}'")]
        MissingToken { path: String },
        #[error("Material color token '{path}' requires a Seed")]
        MissingSeed { path: String },
    }

    pub trait TokenSource {
        type Error;
        fn color(&self, path: &str) -> Result<Color, Self::Error>;
    }

    pub trait TokenValue<S: TokenSource>: Sized {
        fn read(source: &S, path: &str) -> Result<Self, S::Error>;
    }

    impl<S: TokenSource> TokenValue<S> for Color {
        fn read(source: &S, path: &str) -> Result<Self, S::Error> {
            source.color(path)
        }
    }

    #[cfg(feature = "seed")]
    impl TokenSource for ResolvedTheme {
        type Error = BuildError;

        fn color(&self, path: &str) -> Result<Color, Self::Error> {
            let binding =
                self.colors
                    .get(path)
                    .copied()
                    .ok_or_else(|| BuildError::MissingToken {
                        path: path.to_owned(),
                    })?;
            match binding {
                ColorBinding::Color(color) => Ok(color),
                ColorBinding::Material(role) => {
                    let seed = self.seed.ok_or_else(|| BuildError::MissingSeed {
                        path: path.to_owned(),
                    })?;
                    Ok(spectrum_palette::material_colors(seed, self.meta.mode).resolve(role))
                }
            }
        }
    }
}
