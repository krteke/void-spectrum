//! Public facade for the Void Spectrum typed theme-token engine.

pub use spectrum_core::{
    Color, ColorParseError, FontStyle, FontStyleParseError, FontWeight, FontWeightParseError,
    Length, LengthParseError, LengthUnit, LineHeight, LineHeightParseError, Radius,
    RadiusParseError, Rgb, Rgba, ShadowError, ShadowLayer, ThemeMode,
};

#[cfg(feature = "iced")]
pub use spectrum_iced as iced;
#[cfg(feature = "macros")]
pub use spectrum_macros::define_theme_tokens;
#[cfg(feature = "ratatui")]
pub use spectrum_ratatui as ratatui;

#[cfg(feature = "toml")]
pub mod config;
pub mod source;

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
    /// A token exists but cannot be parsed as the requested value type.
    #[error("invalid value for token '{path}': {message}")]
    InvalidTokenValue {
        /// Token path.
        path: String,
        /// Human-readable parse failure.
        message: String,
    },
}
