//! Ratatui adapter behavior tests.

use ratatui::style::{Color as RatatuiColor, Modifier, Style};
use spectrum_core::{Color, FontStyle, FontWeight};
use spectrum_ratatui::RatatuiTextStyle;
use spectrum_ratatui::{RatatuiColorAdapter, RatatuiModifierAdapter, RatatuiStyleAdapter, style};

#[test]
fn converts_rgb_colors_to_ratatui_rgb() {
    assert_eq!(
        Color::new(12, 34, 56).color(),
        RatatuiColor::Rgb(12, 34, 56)
    );
}

#[test]
fn discards_alpha_for_terminal_colors() {
    assert_eq!(
        Color::new_rgba(12, 34, 56, 78).color(),
        RatatuiColor::Rgb(12, 34, 56)
    );
}

#[test]
fn builds_styles_from_optional_colors() {
    let expected = Style::new()
        .fg(RatatuiColor::Rgb(1, 2, 3))
        .bg(RatatuiColor::Rgb(4, 5, 6));

    assert_eq!(
        (Some(Color::new(1, 2, 3)), Some(Color::new(4, 5, 6))).style(),
        expected
    );
    assert_eq!(
        style(Some(Color::new(1, 2, 3)), Some(Color::new(4, 5, 6))),
        expected
    );
}

#[test]
fn maps_font_weight_to_bold_modifier() {
    assert_eq!(FontWeight::MEDIUM.modifier(), Modifier::empty());
    assert_eq!(FontWeight::SEMI_BOLD.modifier(), Modifier::BOLD);
}

#[test]
fn maps_font_style_to_italic_modifier() {
    assert_eq!(FontStyle::Normal.modifier(), Modifier::empty());
    assert_eq!(FontStyle::Italic.modifier(), Modifier::ITALIC);
    assert_eq!(FontStyle::Oblique.modifier(), Modifier::ITALIC);
}

#[test]
fn combines_font_weight_and_style_modifiers() {
    assert_eq!(
        (FontWeight::BOLD, FontStyle::Italic).modifier(),
        Modifier::BOLD | Modifier::ITALIC
    );
}

#[test]
fn builds_text_styles_from_color_and_font_values() {
    let text_style = RatatuiTextStyle {
        foreground: Some(Color::new(1, 2, 3)),
        background: Some(Color::new(4, 5, 6)),
        font_weight: Some(FontWeight::BOLD),
        font_style: Some(FontStyle::Italic),
    };

    assert_eq!(
        text_style.style(),
        Style::new()
            .fg(RatatuiColor::Rgb(1, 2, 3))
            .bg(RatatuiColor::Rgb(4, 5, 6))
            .add_modifier(Modifier::BOLD | Modifier::ITALIC)
    );
}
