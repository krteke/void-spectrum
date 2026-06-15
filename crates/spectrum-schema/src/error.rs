use core::fmt;
use spectrum_core::{ColorParseError, LengthParseError};

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
