use spectrum_core::LengthUnit;
use thiserror::Error;

/// Describes why a Spectrum length cannot be represented by Iced layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum IcedLengthError {
    /// Iced layout lengths do not carry this unit.
    #[error("Iced length does not support '{unit}' units")]
    UnsupportedUnit {
        /// Unsupported Spectrum length unit.
        unit: LengthUnit,
    },
}

/// Describes why a Spectrum radius cannot be represented by Iced borders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum IcedRadiusError {
    /// Iced border radii use pixel units.
    #[error("Iced radius does not support '{unit}' units")]
    UnsupportedUnit {
        /// Unsupported Spectrum radius unit.
        unit: LengthUnit,
    },
}
