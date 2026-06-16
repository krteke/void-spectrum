//! Ratatui adapter behavior tests.

use ratatui::style::{Color as RatatuiColor, Style};
use spectrum_core::Color;
use spectrum_ratatui::{color, style};

#[test]
fn converts_rgb_colors_to_ratatui_rgb() {
    assert_eq!(color(Color::new(12, 34, 56)), RatatuiColor::Rgb(12, 34, 56));
}

#[test]
fn discards_alpha_for_terminal_colors() {
    assert_eq!(
        color(Color::new_rgba(12, 34, 56, 78)),
        RatatuiColor::Rgb(12, 34, 56)
    );
}

#[test]
fn builds_styles_from_optional_colors() {
    assert_eq!(
        style(Some(Color::new(1, 2, 3)), Some(Color::new(4, 5, 6))),
        Style::new()
            .fg(RatatuiColor::Rgb(1, 2, 3))
            .bg(RatatuiColor::Rgb(4, 5, 6))
    );
}
