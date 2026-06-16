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

/// Describes why Spectrum border inputs cannot be represented by Iced borders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum IcedBorderError {
    /// Iced border dimensions use pixel units.
    #[error("Iced border field '{field}' does not support '{unit}' units")]
    UnsupportedUnit {
        /// Border field that used an unsupported unit.
        field: &'static str,
        /// Unsupported Spectrum unit.
        unit: LengthUnit,
    },
}

/// Describes why a Spectrum shadow cannot be represented by Iced shadows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum IcedShadowError {
    /// Iced shadows use pixel units.
    #[error("Iced shadow field '{field}' does not support '{unit}' units")]
    UnsupportedUnit {
        /// Shadow field that used an unsupported unit.
        field: &'static str,
        /// Unsupported Spectrum shadow unit.
        unit: LengthUnit,
    },
    /// Iced shadows do not expose spread radius.
    #[error("Iced shadow does not support non-zero spread")]
    UnsupportedSpread,
}
