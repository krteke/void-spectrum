//! Ratatui conversions for Void Spectrum core values.

use ratatui::style::{Color as RatatuiColor, Style};
use spectrum_core::Color;

/// Converts a Spectrum color into a Ratatui RGB color.
///
/// Ratatui terminal colors do not carry alpha, so RGBA input is composited by
/// discarding alpha and preserving the RGB channels.
#[must_use]
pub const fn color(value: Color) -> RatatuiColor {
    RatatuiColor::Rgb(value.red(), value.green(), value.blue())
}

/// Builds a Ratatui style from optional Spectrum foreground and background
/// colors.
#[must_use]
pub fn style(foreground: Option<Color>, background: Option<Color>) -> Style {
    let mut style = Style::new();
    if let Some(foreground) = foreground {
        style = style.fg(color(foreground));
    }
    if let Some(background) = background {
        style = style.bg(color(background));
    }
    style
}
