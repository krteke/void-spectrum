//! Iced conversions for Void Spectrum core values.

use spectrum_core::{Color, Length, LengthUnit, Radius, ShadowLayer};

mod error;

pub use error::{IcedLengthError, IcedRadiusError, IcedShadowError};

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

/// Converts a Spectrum radius into an Iced border radius.
pub trait IcedRadiusAdapter {
    /// Converts the radius to an Iced border radius.
    fn radius(&self) -> Result<iced_core::border::Radius, IcedRadiusError>;
}

impl IcedRadiusAdapter for Radius {
    fn radius(&self) -> Result<iced_core::border::Radius, IcedRadiusError> {
        let length = self.length();
        match length.unit() {
            LengthUnit::Px => Ok(iced_core::border::Radius::new(length.value())),
            unit => Err(IcedRadiusError::UnsupportedUnit { unit }),
        }
    }
}

/// Converts a Spectrum shadow layer into an Iced shadow.
pub trait IcedShadowAdapter {
    /// Converts the shadow to an Iced shadow.
    fn shadow(&self) -> Result<iced_core::Shadow, IcedShadowError>;
}

impl IcedShadowAdapter for ShadowLayer {
    fn shadow(&self) -> Result<iced_core::Shadow, IcedShadowError> {
        let offset_x = shadow_px("offset_x", self.offset_x())?;
        let offset_y = shadow_px("offset_y", self.offset_y())?;
        let blur_radius = shadow_px("blur", self.blur())?;
        let spread = shadow_px("spread", self.spread())?;
        if spread != 0.0 {
            return Err(IcedShadowError::UnsupportedSpread);
        }

        Ok(iced_core::Shadow {
            color: self.color().color(),
            offset: iced_core::Vector::new(offset_x, offset_y),
            blur_radius,
        })
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

/// Converts a Spectrum radius into an Iced border radius.
pub fn radius(value: Radius) -> Result<iced_core::border::Radius, IcedRadiusError> {
    value.radius()
}

/// Converts a Spectrum shadow layer into an Iced shadow.
pub fn shadow(value: ShadowLayer) -> Result<iced_core::Shadow, IcedShadowError> {
    value.shadow()
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

fn shadow_px(field: &'static str, length: Length) -> Result<f32, IcedShadowError> {
    match length.unit() {
        LengthUnit::Px => Ok(length.value()),
        unit => Err(IcedShadowError::UnsupportedUnit { field, unit }),
    }
}
