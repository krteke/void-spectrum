use core::fmt;
use core::str::FromStr;

use crate::{Length, LengthParseError};

#[derive(Debug, Clone, Copy, PartialEq)]
enum LineHeightValue {
    Multiplier(f32),
    Length(Length),
}

/// A non-negative line height expressed as a multiplier or length.
///
/// ```
/// use spectrum_core::LineHeight;
///
/// let compact: LineHeight = "1.25".parse()?;
/// let fixed: LineHeight = "20px".parse()?;
/// assert_eq!(compact.multiplier_value(), Some(1.25));
/// assert!(fixed.length_value().is_some());
/// # Ok::<(), spectrum_core::LineHeightParseError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineHeight(LineHeightValue);

impl Eq for LineHeight {}

impl LineHeight {
    /// Creates a unitless line-height multiplier.
    pub fn multiplier(value: f32) -> Result<Self, LineHeightParseError> {
        if !value.is_finite() {
            return Err(LineHeightParseError::NonFinite);
        }
        if value < 0.0 {
            return Err(LineHeightParseError::Negative);
        }
        Ok(Self(LineHeightValue::Multiplier(value)))
    }

    /// Creates a line height from an explicit length.
    pub fn length(value: Length) -> Result<Self, LineHeightParseError> {
        if value.value() < 0.0 {
            return Err(LineHeightParseError::Negative);
        }
        Ok(Self(LineHeightValue::Length(value)))
    }

    /// Returns the multiplier when this line height is unitless.
    #[must_use]
    pub const fn multiplier_value(self) -> Option<f32> {
        match self.0 {
            LineHeightValue::Multiplier(value) => Some(value),
            LineHeightValue::Length(_) => None,
        }
    }

    /// Returns the length when this line height has an explicit unit.
    #[must_use]
    pub const fn length_value(self) -> Option<Length> {
        match self.0 {
            LineHeightValue::Length(value) => Some(value),
            LineHeightValue::Multiplier(_) => None,
        }
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            LineHeightValue::Multiplier(value) => value.fmt(formatter),
            LineHeightValue::Length(value) => value.fmt(formatter),
        }
    }
}

impl FromStr for LineHeight {
    type Err = LineHeightParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = input.parse::<f32>() {
            return Self::multiplier(value);
        }
        if let Ok(value) = input.parse::<Length>() {
            return Self::length(value);
        }

        Err(LineHeightParseError::InvalidInput)
    }
}

/// Describes why a line height could not be constructed or parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineHeightParseError {
    /// The explicit length is invalid.
    InvalidLength(LengthParseError),
    /// The input is not a valid `f32` or `Length`.
    InvalidInput,
    /// Line height cannot be negative.
    Negative,
    /// A unitless multiplier must be finite.
    NonFinite,
}

impl fmt::Display for LineHeightParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(error) => write!(formatter, "invalid line height: {error}"),
            Self::InvalidInput => formatter.write_str("invalid line height input"),
            Self::Negative => formatter.write_str("line height cannot be negative"),
            Self::NonFinite => formatter.write_str("line height must be finite"),
        }
    }
}

impl std::error::Error for LineHeightParseError {}
