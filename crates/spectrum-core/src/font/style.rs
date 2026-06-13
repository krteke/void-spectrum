use core::fmt;
use core::str::FromStr;

use crate::font::error::FontStyleParseError;

/// The requested shape of a font face.
///
/// ```
/// use spectrum_core::FontStyle;
///
/// let style: FontStyle = "italic".parse()?;
/// assert_eq!(style, FontStyle::Italic);
/// # Ok::<(), spectrum_core::FontStyleParseError>(())
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    /// The font's upright face.
    #[default]
    Normal,
    /// A purpose-designed italic face.
    Italic,
    /// A mechanically slanted face.
    Oblique,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Normal => "normal",
            Self::Italic => "italic",
            Self::Oblique => "oblique",
        })
    }
}

impl FromStr for FontStyle {
    type Err = FontStyleParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "normal" => Ok(Self::Normal),
            "italic" => Ok(Self::Italic),
            "oblique" => Ok(Self::Oblique),
            _ => Err(FontStyleParseError),
        }
    }
}
