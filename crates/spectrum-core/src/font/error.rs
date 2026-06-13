use core::fmt;

/// Describes why a font weight could not be constructed or parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeightParseError {
    /// The input is not an unsigned integer.
    InvalidNumber,
    /// The value is outside `1..=1000`.
    OutOfRange,
}

impl fmt::Display for FontWeightParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidNumber => "font weight must be an unsigned integer",
            Self::OutOfRange => "font weight must be between 1 and 1000",
        })
    }
}

impl std::error::Error for FontWeightParseError {}

/// Indicates that a font style name is unsupported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontStyleParseError;

impl fmt::Display for FontStyleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("font style must be normal, italic, or oblique")
    }
}

impl std::error::Error for FontStyleParseError {}
