//! Iced adapter behavior tests.

use spectrum_core::Color;
use spectrum_iced::{IcedColorAdapter, color};

#[test]
fn converts_rgb_colors_to_iced_rgba() {
    assert_eq!(
        Color::new(12, 34, 56).color(),
        iced_core::Color::from_rgba8(12, 34, 56, 1.0)
    );
}

#[test]
fn preserves_alpha_for_iced_colors() {
    assert_eq!(
        color(Color::new_rgba(12, 34, 56, 128)),
        iced_core::Color::from_rgba8(12, 34, 56, 128.0 / 255.0)
    );
}
