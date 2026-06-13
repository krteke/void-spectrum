//! Tests for perceptual color transformations.

#![cfg(feature = "color-spaces")]

use palette::{IntoColor, Oklab, Srgb};
use spectrum_core::Color;
use spectrum_palette::ColorExt;

fn lightness(color: Color) -> f32 {
    let rgb = color.rgb();
    let lab: Oklab = Srgb::new(rgb.red(), rgb.green(), rgb.blue())
        .into_linear::<f32>()
        .into_color();
    lab.l
}

#[test]
fn lighten_and_darken_change_perceived_lightness() {
    let color = Color::new(80, 120, 200);

    assert!(lightness(color.lighten(0.4)) > lightness(color));
    assert!(lightness(color.darken(0.4)) < lightness(color));
}

#[test]
fn transformation_amount_is_clamped() {
    let color = Color::new(80, 120, 200);

    assert_eq!(color.lighten(-1.0), color.lighten(0.0));
    assert_eq!(color.darken(2.0), color.darken(1.0));
}

#[test]
fn grayscale_produces_neutral_channels() {
    let gray = Color::new(80, 120, 200).grayscale().rgb();

    assert_eq!(gray.red(), gray.green());
    assert_eq!(gray.green(), gray.blue());
}

#[test]
fn transformations_preserve_alpha() {
    let color = Color::new_rgba(80, 120, 200, 77);

    assert_eq!(color.lighten(0.2).alpha(), 77);
    assert_eq!(color.darken(0.2).alpha(), 77);
    assert_eq!(color.grayscale().alpha(), 77);
    assert_eq!(color.invert_lightness().alpha(), 77);
}

#[test]
fn invert_lightness_swaps_black_and_white() {
    assert_eq!(
        Color::new(0, 0, 0).invert_lightness(),
        Color::new(255, 255, 255)
    );
    assert_eq!(
        Color::new(255, 255, 255).invert_lightness(),
        Color::new(0, 0, 0)
    );
}
