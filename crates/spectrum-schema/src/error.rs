use core::fmt;
use spectrum_core::{
    ColorParseError, FontStyleParseError, FontWeightParseError, LengthParseError,
    LineHeightParseError, RadiusParseError,
};

/// Describes why a color value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorValueParseError {
    /// The direct color is invalid.
    InvalidColor(ColorParseError),
    /// The token reference syntax is invalid.
    InvalidReference,
}

impl fmt::Display for ColorValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidColor(error) => error.fmt(formatter),
            Self::InvalidReference => formatter.write_str("invalid color token reference"),
        }
    }
}

impl std::error::Error for ColorValueParseError {}

/// Describes why a length token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthValueParseError {
    /// The direct length is invalid.
    InvalidLength(LengthParseError),
    /// The token reference syntax is invalid.
    InvalidReference,
}

impl fmt::Display for LengthValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(error) => error.fmt(formatter),
            Self::InvalidReference => formatter.write_str("invalid length token reference"),
        }
    }
}

impl std::error::Error for LengthValueParseError {}

/// Describes why a radius token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiusValueParseError {
    /// The direct radius is invalid.
    InvalidRadius(RadiusParseError),
    /// The token reference syntax is invalid.
    InvalidReference,
}

impl fmt::Display for RadiusValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRadius(error) => error.fmt(formatter),
            Self::InvalidReference => formatter.write_str("invalid radius token reference"),
        }
    }
}

impl std::error::Error for RadiusValueParseError {}

/// Describes why a font-weight token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeightValueParseError {
    /// The direct font weight is invalid.
    InvalidFontWeight(FontWeightParseError),
    /// The token reference syntax is invalid.
    InvalidReference,
}

impl fmt::Display for FontWeightValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFontWeight(error) => error.fmt(formatter),
            Self::InvalidReference => formatter.write_str("invalid font-weight token reference"),
        }
    }
}

impl std::error::Error for FontWeightValueParseError {}

/// Describes why a font-style token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyleValueParseError {
    /// The direct font style is invalid.
    InvalidFontStyle(FontStyleParseError),
    /// The token reference syntax is invalid.
    InvalidReference,
}

impl fmt::Display for FontStyleValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFontStyle(error) => error.fmt(formatter),
            Self::InvalidReference => formatter.write_str("invalid font-style token reference"),
        }
    }
}

impl std::error::Error for FontStyleValueParseError {}

/// Describes why a line-height token value could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineHeightValueParseError {
    /// The direct line height is invalid.
    InvalidLineHeight(LineHeightParseError),
    /// The token reference syntax is invalid.
    InvalidReference,
}

impl fmt::Display for LineHeightValueParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLineHeight(error) => error.fmt(formatter),
            Self::InvalidReference => formatter.write_str("invalid line-height token reference"),
        }
    }
}

impl std::error::Error for LineHeightValueParseError {}
