use core::fmt;

/// Describes why a hexadecimal color could not be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorParseError {
    /// The leading `#` is absent.
    MissingHash,
    /// The color does not contain exactly six or eight hexadecimal digits.
    InvalidLength,
    /// At least one character is not a hexadecimal digit.
    InvalidDigit,
}

impl fmt::Display for ColorParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingHash => "color must start with '#'",
            Self::InvalidLength => "color must contain six or eight hexadecimal digits",
            Self::InvalidDigit => "color contains a non-hexadecimal digit",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ColorParseError {}
