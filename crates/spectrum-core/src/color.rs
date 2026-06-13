use core::fmt;
use core::str::FromStr;

mod error;
mod rgb;
mod rgba;

pub use error::ColorParseError;
pub use rgb::Rgb;
pub use rgba::Rgba;

/// An RGB color with optional alpha transparency.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// An RGB color with an alpha channel.
    Rgb(Rgb),
    /// An RGB color with an alpha channel.
    Rgba(Rgba),
}

impl Color {
    /// Creates an opaque color from RGB channels.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(Rgb::new(r, g, b))
    }

    /// Creates a color from RGBA channels.
    #[must_use]
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgba(Rgba::new(r, g, b, a))
    }

    /// Returns the red channel.
    #[must_use]
    pub const fn red(self) -> u8 {
        match self {
            Self::Rgb(r) => r.red(),
            Self::Rgba(r) => r.red(),
        }
    }

    /// Returns the green channel.
    #[must_use]
    pub const fn green(self) -> u8 {
        match self {
            Self::Rgb(r) => r.green(),
            Self::Rgba(r) => r.green(),
        }
    }

    /// Returns the blue channel.
    #[must_use]
    pub const fn blue(self) -> u8 {
        match self {
            Self::Rgb(r) => r.blue(),
            Self::Rgba(r) => r.blue(),
        }
    }

    /// Returns the alpha channel, using `255` for opaque RGB colors.
    #[must_use]
    pub const fn alpha(self) -> u8 {
        match self {
            Self::Rgb(_) => u8::MAX,
            Self::Rgba(r) => r.alpha(),
        }
    }

    /// Converts the color to an RGB color, discarding the alpha channel.
    #[must_use]
    pub const fn rgb(self) -> Rgb {
        match self {
            Self::Rgb(r) => r,
            Self::Rgba(r) => r.into_rgb(),
        }
    }

    /// Converts the color to an RGBA color, using `255` for the alpha channel.
    #[must_use]
    pub const fn rgba(self) -> Rgba {
        match self {
            Self::Rgb(r) => r.into_rgba(),
            Self::Rgba(r) => r,
        }
    }

    /// Parses a color implemented by the `Hex` trait.
    ///
    /// It can accept a `u32` or a `&str`.
    pub fn from_hex<T: Hex>(value: T) -> Result<Self, ColorParseError> {
        value.to_color()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rgb(r) => r.fmt(f),
            Self::Rgba(r) => r.fmt(f),
        }
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for u32 {}
    impl Sealed for &str {}
}

pub trait Hex: private::Sealed {
    fn to_color(self) -> Result<Color, ColorParseError>;
}

impl Hex for u32 {
    fn to_color(self) -> Result<Color, ColorParseError> {
        Ok((self).into())
    }
}

impl Hex for &str {
    fn to_color(self) -> Result<Color, ColorParseError> {
        Color::from_str(self)
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Color::from_str(value)
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    /// Parses a color encoded as `#RRGGBB` or `#RRGGBBAA`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digits = value
            .strip_prefix('#')
            .ok_or(ColorParseError::MissingHash)?;
        if !matches!(digits.len(), 6 | 8) {
            return Err(ColorParseError::InvalidLength);
        }

        let channel = |range| {
            u8::from_str_radix(&digits[range], 16).map_err(|_| ColorParseError::InvalidDigit)
        };
        let (r, g, b) = (channel(0..2)?, channel(2..4)?, channel(4..6)?);
        match digits.len() {
            6 => Ok(Self::new(r, g, b)),
            8 => Ok(Self::new_rgba(r, g, b, channel(6..8)?)),
            _ => unreachable!(),
        }
    }
}

impl From<u32> for Color {
    /// Converts a packed `u32` color value to an RGBA color.
    ///
    /// The color value is expected to be in the format `0xRRGGBBAA`.
    fn from(value: u32) -> Self {
        let bytes = value.to_be_bytes();
        let r = bytes[0];
        let g = bytes[1];
        let b = bytes[2];
        let a = bytes[3];

        Self::Rgba(Rgba::new(r, g, b, a))
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        Self::Rgba(value)
    }
}
