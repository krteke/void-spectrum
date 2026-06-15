use core::fmt;
use core::str::FromStr;

use crate::{Length, LengthParseError};

/// A non-negative corner radius.
///
/// ```
/// use spectrum_core::Radius;
///
/// let radius: Radius = "8px".parse()?;
/// assert_eq!(radius.to_string(), "8px");
/// # Ok::<(), spectrum_core::RadiusParseError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radius(Length);

impl Eq for Radius {}

impl Radius {
    /// Creates a radius from a validated length.
    pub fn new(length: Length) -> Result<Self, RadiusParseError> {
        if length.value() < 0.0 {
            return Err(RadiusParseError::Negative);
        }
        Ok(Self(length))
    }

    /// Returns the underlying length.
    #[must_use]
    pub const fn length(self) -> Length {
        self.0
    }
}

impl fmt::Display for Radius {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for Radius {
    type Err = RadiusParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::new(input.parse().map_err(RadiusParseError::InvalidLength)?)
    }
}

/// Describes why a radius could not be constructed or parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiusParseError {
    /// The underlying length is invalid.
    InvalidLength(LengthParseError),
    /// Radius values cannot be negative.
    Negative,
}

impl fmt::Display for RadiusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(error) => write!(formatter, "invalid radius: {error}"),
            Self::Negative => formatter.write_str("radius cannot be negative"),
        }
    }
}

impl std::error::Error for RadiusParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidLength(error) => Some(error),
            Self::Negative => None,
        }
    }
}
