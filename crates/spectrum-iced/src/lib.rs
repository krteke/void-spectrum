//! Iced conversions for Void Spectrum core values.

use core::fmt;
use spectrum_core::{Color, Length, LengthUnit};

mod error;

pub use error::IcedLengthError;

/// Converts a Spectrum color into an Iced color.
pub trait IcedColorAdapter {
    /// Converts the color to an Iced color.
    #[must_use]
    fn color(&self) -> iced_core::Color;
}

impl IcedColorAdapter for Color {
    fn color(&self) -> iced_core::Color {
        iced_core::Color::from_rgba8(self.red(), self.green(), self.blue(), alpha(self.alpha()))
    }
}

/// Converts a Spectrum length into an Iced length.
pub trait IcedLengthAdapter {
    /// Converts the length to an Iced length.
    fn length(&self) -> Result<iced_core::Length, IcedLengthError>;
}

impl IcedLengthAdapter for Length {
    fn length(&self) -> Result<iced_core::Length, IcedLengthError> {
        match self.unit() {
            LengthUnit::Px => Ok(iced_core::Length::Fixed(self.value())),
            unit => Err(IcedLengthError::UnsupportedUnit { unit }),
        }
    }
}

/// Converts a Spectrum color into an Iced color.
#[must_use]
pub fn color(value: Color) -> iced_core::Color {
    value.color()
}

/// Converts a Spectrum length into an Iced length.
pub fn length(value: Length) -> Result<iced_core::Length, IcedLengthError> {
    value.length()
}

fn alpha(value: u8) -> f32 {
    if value == u8::MAX {
        1.0
    } else if value == 0 {
        0.0
    } else {
        f32::from(value) / 255.0
    }
}
