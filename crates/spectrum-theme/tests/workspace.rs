//! Tests for stable facade exports.

use spectrum_theme::Color;

#[test]
fn facade_exports_stable_core_types() {
    assert_eq!(Color::new(80, 120, 200).to_string(), "#5078c8");
}

#[cfg(feature = "ratatui")]
#[test]
fn facade_exports_ratatui_adapter() {
    assert_eq!(
        format!("{:?}", spectrum_theme::ratatui::color(Color::new(1, 2, 3))),
        "Rgb(1, 2, 3)"
    );
}

#[cfg(feature = "iced")]
#[test]
fn facade_exports_iced_adapter() {
    assert_eq!(
        format!("{:?}", spectrum_theme::iced::color(Color::new(1, 2, 3))),
        "Color { r: 0.003921569, g: 0.007843138, b: 0.011764706, a: 1.0 }"
    );
}
