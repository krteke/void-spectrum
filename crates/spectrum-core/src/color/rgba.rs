use core::fmt;

use crate::Rgb;

/// An RGB color with an alpha channel.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rgba {
    /// Red channel.
    r: u8,
    /// Green channel.
    g: u8,
    /// Blue channel.
    b: u8,
    /// Alpha channel.
    a: u8,
}

impl Rgba {
    /// Creates a new RGBA color with the given red, green, blue, and alpha channel values.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
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

    /// Returns the alpha channel value.
    #[must_use]
    pub const fn alpha(self) -> u8 {
        self.a
    }

    /// Converts an RGBA color to an RGB color by dropping the alpha channel.
    #[must_use]
    pub const fn into_rgb(self) -> Rgb {
        Rgb::new(self.r, self.g, self.b)
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r, self.g, self.b, self.a
        )
    }
}

impl From<Rgb> for Rgba {
    /// Converts an RGB color to an RGBA color with a fully opaque alpha channel.
    fn from(value: Rgb) -> Self {
        Self {
            r: value.red(),
            g: value.green(),
            b: value.blue(),
            a: u8::MAX,
        }
    }
}
