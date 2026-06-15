use core::fmt;
use core::str::FromStr;

/// A unit supported by platform-independent theme dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LengthUnit {
    /// Device-independent pixel.
    Px,
    /// Root font-relative unit.
    Rem,
    /// Current font-relative unit.
    Em,
    /// Percentage of a target-defined reference value.
    Percent,
}

impl fmt::Display for LengthUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Px => "px",
            Self::Rem => "rem",
            Self::Em => "em",
            Self::Percent => "%",
        })
    }
}

/// A finite dimension paired with an explicit unit.
///
/// ```
/// use spectrum_core::{Length, LengthUnit};
///
/// let gap: Length = "1.5rem".parse()?;
/// assert_eq!(gap.unit(), LengthUnit::Rem);
/// assert_eq!(gap.to_string(), "1.5rem");
/// # Ok::<(), spectrum_core::LengthParseError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Length {
    value: f32,
    unit: LengthUnit,
}

impl Eq for Length {}

impl Length {
    /// Creates a length, rejecting non-finite values.
    pub fn new(value: f32, unit: LengthUnit) -> Result<Self, LengthParseError> {
        if !value.is_finite() {
            return Err(LengthParseError::NonFinite);
        }
        Ok(Self { value, unit })
    }

    /// Returns the numeric component.
    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }

    /// Returns the unit component.
    #[must_use]
    pub const fn unit(self) -> LengthUnit {
        self.unit
    }
}

impl fmt::Display for Length {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}{}", self.value, self.unit)
    }
}

impl FromStr for Length {
    type Err = LengthParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (number, unit) = [
            ("rem", LengthUnit::Rem),
            ("px", LengthUnit::Px),
            ("em", LengthUnit::Em),
            ("%", LengthUnit::Percent),
        ]
        .into_iter()
        .find_map(|(suffix, unit)| input.strip_suffix(suffix).map(|number| (number, unit)))
        .ok_or(LengthParseError::InvalidUnit)?;
        let value = number
            .parse::<f32>()
            .map_err(|_| LengthParseError::InvalidNumber)?;
        Self::new(value, unit)
    }
}

/// Describes why a length could not be constructed or parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthParseError {
    /// The numeric component is malformed.
    InvalidNumber,
    /// The unit is absent or unsupported.
    InvalidUnit,
    /// The numeric component is NaN or infinite.
    NonFinite,
}

impl fmt::Display for LengthParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidNumber => "length contains an invalid number",
            Self::InvalidUnit => "length contains an unsupported unit",
            Self::NonFinite => "length must be finite",
        })
    }
}

impl std::error::Error for LengthParseError {}
