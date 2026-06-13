use core::fmt;
use core::str::FromStr;

/// An opaque color represented by red, green, and blue channels.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

impl Color {
    /// Creates a color from RGB channels.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parses a color encoded as `#RRGGBB`.
    pub fn from_hex(value: &str) -> Result<Self, ColorParseError> {
        let digits = value
            .strip_prefix('#')
            .ok_or(ColorParseError::MissingHash)?;
        if digits.len() != 6 {
            return Err(ColorParseError::InvalidLength);
        }

        let channel = |range| {
            u8::from_str_radix(&digits[range], 16).map_err(|_| ColorParseError::InvalidDigit)
        };

        Ok(Self::new(channel(0..2)?, channel(2..4)?, channel(4..6)?))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_hex(value)
    }
}

/// Describes why a hexadecimal color could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorParseError {
    /// The leading `#` is absent.
    MissingHash,
    /// The color does not contain exactly six hexadecimal digits.
    InvalidLength,
    /// At least one character is not a hexadecimal digit.
    InvalidDigit,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingHash => "color must start with '#'",
            Self::InvalidLength => "color must contain exactly six hexadecimal digits",
            Self::InvalidDigit => "color contains a non-hexadecimal digit",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ColorParseError {}
