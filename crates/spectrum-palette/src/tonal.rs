use material_colors::{color::Argb, hct::Hct, palette::TonalPalette as MaterialTonalPalette};
use spectrum_core::Color;

/// A Material HCT tonal palette derived from a seed color.
#[derive(Debug, Clone, Copy)]
pub struct TonalPalette {
    inner: MaterialTonalPalette,
}

impl TonalPalette {
    /// Creates a palette using the seed's HCT hue and chroma.
    ///
    /// Alpha is ignored because Material tonal palettes produce opaque colors.
    #[must_use]
    pub fn from_seed(seed: Color) -> Self {
        let argb = argb_from_color(seed);
        let hct: Hct = argb.into();
        Self {
            inner: MaterialTonalPalette::from_hct(hct),
        }
    }

    /// Returns an opaque color at the requested tone, clamped to `0..=100`.
    #[must_use]
    pub fn tone(self, tone: u8) -> Color {
        let color = self.inner.tone(i32::from(tone.min(100)));
        Color::new(color.red, color.green, color.blue)
    }
}

pub(crate) const fn argb_from_color(color: Color) -> Argb {
    Argb::new(u8::MAX, color.red(), color.green(), color.blue())
}

impl From<MaterialTonalPalette> for TonalPalette {
    fn from(inner: MaterialTonalPalette) -> Self {
        Self { inner }
    }
}
