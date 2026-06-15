use core::fmt;

use crate::{Color, Length};

/// A single platform-independent shadow layer.
///
/// ```
/// use spectrum_core::{Color, Length, LengthUnit, ShadowLayer};
///
/// let px = |value| Length::new(value, LengthUnit::Px).expect("finite");
/// let shadow = ShadowLayer::new(
///     Color::new_rgba(0, 0, 0, 128),
///     px(0.0),
///     px(4.0),
///     px(8.0),
///     px(0.0),
/// )?;
/// assert_eq!(shadow.blur().value(), 8.0);
/// # Ok::<(), spectrum_core::ShadowError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowLayer {
    color: Color,
    offset_x: Length,
    offset_y: Length,
    blur: Length,
    spread: Length,
}

impl Eq for ShadowLayer {}

impl ShadowLayer {
    /// Creates a shadow layer, rejecting a negative blur radius.
    pub fn new(
        color: Color,
        offset_x: Length,
        offset_y: Length,
        blur: Length,
        spread: Length,
    ) -> Result<Self, ShadowError> {
        if blur.value() < 0.0 {
            return Err(ShadowError::NegativeBlur);
        }
        Ok(Self {
            color,
            offset_x,
            offset_y,
            blur,
            spread,
        })
    }

    /// Returns the shadow color.
    #[must_use]
    pub const fn color(self) -> Color {
        self.color
    }

    /// Returns the horizontal offset.
    #[must_use]
    pub const fn offset_x(self) -> Length {
        self.offset_x
    }

    /// Returns the vertical offset.
    #[must_use]
    pub const fn offset_y(self) -> Length {
        self.offset_y
    }

    /// Returns the blur radius.
    #[must_use]
    pub const fn blur(self) -> Length {
        self.blur
    }

    /// Returns the spread radius.
    #[must_use]
    pub const fn spread(self) -> Length {
        self.spread
    }
}

/// Describes why a shadow layer could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadowError {
    /// Blur radii cannot be negative.
    NegativeBlur,
}

impl fmt::Display for ShadowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("shadow blur radius cannot be negative")
    }
}

impl std::error::Error for ShadowError {}
