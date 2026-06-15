use material_colors::palette::CorePalette as MaterialCorePalette;
use spectrum_core::Color;

use crate::tonal::{TonalPalette, argb_from_color};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ColorDomains {
    pub(crate) primary: TonalPalette,
    pub(crate) secondary: TonalPalette,
    pub(crate) tertiary: TonalPalette,
    pub(crate) neutral: TonalPalette,
    pub(crate) neutral_variant: TonalPalette,
    pub(crate) error: TonalPalette,
}

impl ColorDomains {
    pub(crate) fn from_seed(seed: Color) -> Self {
        let palette = MaterialCorePalette::of(argb_from_color(seed));

        Self {
            primary: palette.primary.into(),
            secondary: palette.secondary.into(),
            tertiary: palette.tertiary.into(),
            neutral: palette.neutral.into(),
            neutral_variant: palette.neutral_variant.into(),
            error: palette.error.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use spectrum_core::Color;

    use super::ColorDomains;

    #[test]
    fn blue_seed_populates_each_material_color_domain() {
        let domains = ColorDomains::from_seed(Color::new(0, 0, 255));

        assert_eq!(domains.primary.tone(40), Color::new(0x34, 0x3d, 0xff));
        assert_eq!(domains.secondary.tone(40), Color::new(0x5c, 0x5d, 0x72));
        assert_eq!(domains.tertiary.tone(40), Color::new(0x78, 0x53, 0x6b));
        assert_eq!(domains.neutral.tone(40), Color::new(0x5f, 0x5e, 0x62));
        assert_eq!(
            domains.neutral_variant.tone(40),
            Color::new(0x5e, 0x5d, 0x67)
        );
        assert_eq!(domains.error.tone(40), Color::new(0xba, 0x1a, 0x1a));
    }

    #[test]
    fn seed_alpha_does_not_change_color_domains() {
        let opaque = ColorDomains::from_seed(Color::new(80, 120, 200));
        let transparent = ColorDomains::from_seed(Color::new_rgba(80, 120, 200, 1));

        assert_eq!(transparent.primary.tone(40), opaque.primary.tone(40));
        assert_eq!(transparent.neutral.tone(90), opaque.neutral.tone(90));
        assert_eq!(transparent.error.tone(80), opaque.error.tone(80));
    }
}
