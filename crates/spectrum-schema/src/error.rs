use spectrum_core::{
    ColorParseError, FontStyleParseError, FontWeightParseError, LengthParseError,
    LineHeightParseError, RadiusParseError,
};
use thiserror::Error;

/// Describes why a color value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ColorValueParseError {
    /// The direct color is invalid.
    #[error(transparent)]
    InvalidColor(#[from] ColorParseError),
    /// The token reference syntax is invalid.
    #[error("invalid color token reference")]
    InvalidReference,
}

/// Describes why a length token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LengthValueParseError {
    /// The direct length is invalid.
    #[error(transparent)]
    InvalidLength(#[from] LengthParseError),
    /// The token reference syntax is invalid.
    #[error("invalid length token reference")]
    InvalidReference,
}

/// Describes why a radius token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum RadiusValueParseError {
    /// The direct radius is invalid.
    #[error(transparent)]
    InvalidRadius(#[from] RadiusParseError),
    /// The token reference syntax is invalid.
    #[error("invalid radius token reference")]
    InvalidReference,
}

/// Describes why a font-weight token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum FontWeightValueParseError {
    /// The direct font weight is invalid.
    #[error(transparent)]
    InvalidFontWeight(#[from] FontWeightParseError),
    /// The token reference syntax is invalid.
    #[error("invalid font-weight token reference")]
    InvalidReference,
}

/// Describes why a font-style token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum FontStyleValueParseError {
    /// The direct font style is invalid.
    #[error(transparent)]
    InvalidFontStyle(#[from] FontStyleParseError),
    /// The token reference syntax is invalid.
    #[error("invalid font-style token reference")]
    InvalidReference,
}

/// Describes why a line-height token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LineHeightValueParseError {
    /// The direct line height is invalid.
    #[error(transparent)]
    InvalidLineHeight(#[from] LineHeightParseError),
    /// The token reference syntax is invalid.
    #[error("invalid line-height token reference")]
    InvalidReference,
}
