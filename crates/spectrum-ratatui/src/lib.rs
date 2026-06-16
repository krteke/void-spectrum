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

/// Spectrum text styling values that can be converted to a Ratatui style.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RatatuiTextStyle {
    /// Foreground text color.
    pub foreground: Option<Color>,
    /// Background text color.
    pub background: Option<Color>,
    /// Font weight used for terminal emphasis.
    pub font_weight: Option<FontWeight>,
    /// Font style used for terminal emphasis.
    pub font_style: Option<FontStyle>,
}

impl RatatuiTextStyle {
    /// Creates a new [`RatatuiTextStyle`] with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            foreground: None,
            background: None,
            font_weight: None,
            font_style: None,
        }
    }

    /// Sets the foreground color of the text style.
    #[must_use]
    pub const fn with_fg(self, foreground: Color) -> Self {
        Self {
            foreground: Some(foreground),
            ..self
        }
    }

    /// Sets the background color of the text style.
    #[must_use]
    pub const fn with_bg(self, background: Color) -> Self {
        Self {
            background: Some(background),
            ..self
        }
    }

    /// Set the font weight of the text style.
    #[must_use]
    pub const fn with_weight(self, font_weight: FontWeight) -> Self {
        Self {
            font_weight: Some(font_weight),
            ..self
        }
    }

    /// Set the font style of the text style.
    #[must_use]
    pub const fn with_style(self, font_style: FontStyle) -> Self {
        Self {
            font_style: Some(font_style),
            ..self
        }
    }
}

impl RatatuiStyleAdapter for RatatuiTextStyle {
    fn style(&self) -> Style {
        let mut style = (self.foreground, self.background).style();
        if let Some(font_weight) = self.font_weight {
            style = style.add_modifier(font_weight.modifier());
        }
        if let Some(font_style) = self.font_style {
            style = style.add_modifier(font_style.modifier());
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
