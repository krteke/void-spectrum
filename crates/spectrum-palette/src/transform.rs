use palette::{Clamp, Darken, IntoColor, Lighten, LinSrgb, Oklab, Srgb};
use spectrum_core::{Color, Rgb};

/// Perceptual color transformations backed by Oklab.
///
/// ```
/// use spectrum_core::Color;
/// use spectrum_palette::ColorExt;
///
/// let raised = Color::new(40, 80, 120).lighten(0.25);
/// assert_eq!(raised.alpha(), 255);
/// ```
pub trait ColorExt {
    /// Increases perceived lightness by a factor in `0..=1`.
    #[must_use]
    fn lighten(self, amount: f32) -> Color;

    /// Decreases perceived lightness by a factor in `0..=1`.
    #[must_use]
    fn darken(self, amount: f32) -> Color;

    /// Removes chroma while preserving perceived lightness.
    #[must_use]
    fn grayscale(self) -> Color;

    /// Reverses perceived lightness while preserving chroma.
    #[must_use]
    fn invert_lightness(self) -> Color;
}

impl ColorExt for Color {
    fn lighten(self, amount: f32) -> Color {
        from_oklab(to_oklab(self).lighten(amount.clamp(0.0, 1.0)), self)
    }

    fn darken(self, amount: f32) -> Color {
        from_oklab(to_oklab(self).darken(amount.clamp(0.0, 1.0)), self)
    }

    fn grayscale(self) -> Color {
        let color = to_oklab(self);
        from_oklab(Oklab::new(color.l, 0.0, 0.0), self)
    }

    fn invert_lightness(self) -> Color {
        let mut color = to_oklab(self);
        color.l = 1.0 - color.l;
        from_oklab(color, self)
    }
}

fn to_oklab(color: Color) -> Oklab {
    let rgb = color.rgb();
    Srgb::new(rgb.red(), rgb.green(), rgb.blue())
        .into_linear::<f32>()
        .into_color()
}

fn from_oklab(color: Oklab, source: Color) -> Color {
    let linear: LinSrgb = color.into_color();
    let encoded: Srgb<f32> = Srgb::from_linear(linear);
    let rgb: Srgb<u8> = encoded.clamp().into_format();
    let opaque = Rgb::new(rgb.red, rgb.green, rgb.blue);

    match source {
        Color::Rgb(_) => opaque.into(),
        Color::Rgba(rgba) => {
            Color::new_rgba(opaque.red(), opaque.green(), opaque.blue(), rgba.alpha())
        }
    }
}
