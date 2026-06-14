//! Tests for Seed Color to semantic color mapping.

#![cfg(feature = "seed")]

use spectrum_core::{Color, ThemeMode};
use spectrum_palette::semantic_colors;

#[test]
fn maps_blue_seed_to_light_core_semantics() {
    let colors = semantic_colors(Color::new(0, 0, 255), ThemeMode::Light);

    assert_eq!(colors.primary, Color::new(0x34, 0x3d, 0xff));
    assert_eq!(colors.on_primary, Color::new(0xff, 0xff, 0xff));
    assert_eq!(colors.background, colors.surface);
    assert_eq!(colors.on_background, colors.on_surface);
    assert_eq!(colors.error, Color::new(0xba, 0x1a, 0x1a));
}

#[test]
fn maps_blue_seed_to_dark_core_semantics() {
    let colors = semantic_colors(Color::new(0, 0, 255), ThemeMode::Dark);

    assert_eq!(colors.primary, Color::new(0xbe, 0xc2, 0xff));
    assert_eq!(colors.on_primary, Color::new(0x00, 0x01, 0xac));
    assert_eq!(colors.background, colors.surface);
    assert_eq!(colors.on_background, colors.on_surface);
    assert_eq!(colors.error, Color::new(0xff, 0xb4, 0xab));
}
