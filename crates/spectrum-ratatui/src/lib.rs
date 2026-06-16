//! Ratatui conversions for Void Spectrum core values.

use ratatui::style::{Color as RatatuiColor, Modifier, Style};
use spectrum_core::{Color, FontStyle, FontWeight};

/// Converts a Spectrum color into a Ratatui RGB color.
pub trait RatatuiColorAdapter {
    /// Converts the color to a Ratatui RGB color.
    #[must_use]
    fn color(&self) -> RatatuiColor;
}

impl RatatuiColorAdapter for Color {
    fn color(&self) -> RatatuiColor {
        RatatuiColor::Rgb(self.red(), self.green(), self.blue())
    }
}

/// Converts a Spectrum-backed style description into a Ratatui style.
pub trait RatatuiStyleAdapter {
    /// Converts the value to a Ratatui style.
    #[must_use]
    fn style(&self) -> Style;
}

impl RatatuiStyleAdapter for (Option<Color>, Option<Color>) {
    fn style(&self) -> Style {
        let mut style = Style::new();
        if let Some(foreground) = self.0 {
            style = style.fg(foreground.color());
        }
        if let Some(background) = self.1 {
            style = style.bg(background.color());
        }
        style
    }
}

/// Converts Spectrum text emphasis values into Ratatui modifiers.
pub trait RatatuiModifierAdapter {
    /// Converts the value to a Ratatui modifier set.
    #[must_use]
    fn modifier(&self) -> Modifier;
}

impl RatatuiModifierAdapter for FontWeight {
    fn modifier(&self) -> Modifier {
        if *self >= FontWeight::SEMI_BOLD {
            Modifier::BOLD
        } else {
            Modifier::empty()
        }
    }
}

impl RatatuiModifierAdapter for FontStyle {
    fn modifier(&self) -> Modifier {
        match self {
            Self::Normal => Modifier::empty(),
            Self::Italic | Self::Oblique => Modifier::ITALIC,
        }
    }
}

impl RatatuiModifierAdapter for (FontWeight, FontStyle) {
    fn modifier(&self) -> Modifier {
        self.0.modifier() | self.1.modifier()
    }
}

/// Converts a Spectrum color into a Ratatui RGB color.
#[must_use]
pub fn color(value: Color) -> RatatuiColor {
    value.color()
}

/// Builds a Ratatui style from optional Spectrum foreground and background
/// colors.
#[must_use]
pub fn style(foreground: Option<Color>, background: Option<Color>) -> Style {
    (foreground, background).style()
}
