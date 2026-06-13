use core::fmt;
use std::str::FromStr;

use crate::font::error::FontWeightParseError;

/// A font weight in the OpenType range `1..=1000`.
///
/// ```
/// use spectrum_core::FontWeight;
///
/// let weight: FontWeight = "650".parse()?;
/// assert_eq!(weight.value(), 650);
/// # Ok::<(), spectrum_core::FontWeightParseError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FontWeight(u16);

impl FontWeight {
    /// Thin weight.
    pub const THIN: Self = Self(100);
    /// Extra-light weight.
    pub const EXTRA_LIGHT: Self = Self(200);
    /// Light weight.
    pub const LIGHT: Self = Self(300);
    /// Normal weight.
    pub const NORMAL: Self = Self(400);
    /// Medium weight.
    pub const MEDIUM: Self = Self(500);
    /// Semi-bold weight.
    pub const SEMI_BOLD: Self = Self(600);
    /// Bold weight.
    pub const BOLD: Self = Self(700);
    /// Extra-bold weight.
    pub const EXTRA_BOLD: Self = Self(800);
    /// Black weight.
    pub const BLACK: Self = Self(900);

    /// Creates a font weight within the OpenType range.
    pub const fn new(value: u16) -> Result<Self, FontWeightParseError> {
        if value == 0 || value > 1000 {
            return Err(FontWeightParseError::OutOfRange);
        }
        Ok(Self(value))
    }

    /// Returns the numeric font weight.
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl fmt::Display for FontWeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for FontWeight {
    type Err = FontWeightParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = input
            .parse()
            .map_err(|_| FontWeightParseError::InvalidNumber)?;
        Self::new(value)
    }
}
