//! Tests for seed-derived tonal palettes.

#![cfg(feature = "seed")]

use spectrum_core::Color;
use spectrum_palette::TonalPalette;

#[test]
fn blue_seed_matches_material_hct_reference_tones() {
    let palette = TonalPalette::from_seed(Color::new(0, 0, 255));

    assert_eq!(palette.tone(0), Color::new(0x00, 0x00, 0x00));
    assert_eq!(palette.tone(40), Color::new(0x34, 0x3d, 0xff));
    assert_eq!(palette.tone(80), Color::new(0xbe, 0xc2, 0xff));
    assert_eq!(palette.tone(100), Color::new(0xff, 0xff, 0xff));
}

#[test]
fn tone_values_above_one_hundred_are_clamped() {
    let palette = TonalPalette::from_seed(Color::new(80, 120, 200));

    assert_eq!(palette.tone(200), palette.tone(100));
}

#[test]
fn seed_alpha_does_not_affect_material_palette() {
    let opaque = TonalPalette::from_seed(Color::new(80, 120, 200));
    let transparent = TonalPalette::from_seed(Color::new_rgba(80, 120, 200, 77));

    assert_eq!(transparent.tone(50), opaque.tone(50));
    assert_eq!(transparent.tone(50).alpha(), u8::MAX);
}
