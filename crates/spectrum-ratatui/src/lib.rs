//! Ratatui conversions for Void Spectrum core values.

use ratatui::style::{Color as RatatuiColor, Style};
use spectrum_core::Color;

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
