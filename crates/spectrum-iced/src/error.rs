/// Describes why a Spectrum length cannot be represented by Iced layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcedLengthError {
    /// Iced layout lengths do not carry this unit.
    UnsupportedUnit {
        /// Unsupported Spectrum length unit.
        unit: LengthUnit,
    },
}

impl fmt::Display for IcedLengthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedUnit { unit } => {
                write!(formatter, "Iced length does not support '{unit}' units")
            }
        }
    }
}

impl std::error::Error for IcedLengthError {}
