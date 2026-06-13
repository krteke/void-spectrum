use core::fmt;

use crate::Rgba;

/// An opaque RGB color.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgb {
    /// Red channel.
    r: u8,
    /// Green channel.
    g: u8,
    /// Blue channel.
    b: u8,
}

impl Rgb {
    /// Creates a new RGB color with the given red, green, and blue channel values.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the red channel value.
    #[must_use]
    pub const fn red(self) -> u8 {
        self.r
    }

    /// Returns the green channel value.
    #[must_use]
    pub const fn green(self) -> u8 {
        self.g
    }

    /// Returns the blue channel value.
    #[must_use]
    pub const fn blue(self) -> u8 {
        self.b
    }

    /// Converts an RGBA color to an RGB color by discarding the alpha channel.
    #[must_use]
    pub const fn into_rgba(self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, u8::MAX)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl From<Rgba> for Rgb {
    /// Converts an RGBA color to an RGB color by discarding the alpha channel.
    fn from(value: Rgba) -> Self {
        Self {
            r: value.red(),
            g: value.green(),
            b: value.blue(),
        }
    }
}
