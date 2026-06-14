//! Tests for Seed Color to semantic color mapping.

#![cfg(feature = "seed")]

use material_colors::{color::Argb, palette::CorePalette};
use spectrum_core::{Color, ThemeMode};
use spectrum_palette::{MaterialColor, material_colors, semantic_colors};

fn color(value: Argb) -> Color {
    Color::new(value.red, value.green, value.blue)
}

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

#[test]
fn material_roles_parse_and_select_generated_colors() {
    let colors = material_colors(Color::new(0, 0, 255), ThemeMode::Light);

    assert_eq!(
        colors.resolve(MaterialColor::Primary),
        Color::new(0x34, 0x3d, 0xff)
    );
    assert_eq!(
        MaterialColor::from_name("primary"),
        Some(MaterialColor::Primary)
    );
    assert_eq!(
        MaterialColor::from_name("error_container"),
        Some(MaterialColor::ErrorContainer)
    );
    assert_eq!(MaterialColor::from_name("unknown"), None);
}

#[test]
fn maps_secondary_tertiary_and_container_roles() {
    let seed = Argb::new(255, 0, 0, 255);
    let light = material_colors(Color::new(0, 0, 255), ThemeMode::Light);
    let dark = material_colors(Color::new(0, 0, 255), ThemeMode::Dark);
    let palette = CorePalette::of(seed);
    let roles = [
        (MaterialColor::PrimaryContainer, palette.primary, 90, 30),
        (MaterialColor::OnPrimaryContainer, palette.primary, 30, 90),
        (MaterialColor::Secondary, palette.secondary, 40, 80),
        (MaterialColor::OnSecondary, palette.secondary, 100, 20),
        (MaterialColor::SecondaryContainer, palette.secondary, 90, 30),
        (
            MaterialColor::OnSecondaryContainer,
            palette.secondary,
            30,
            90,
        ),
        (MaterialColor::Tertiary, palette.tertiary, 40, 80),
        (MaterialColor::OnTertiary, palette.tertiary, 100, 20),
        (MaterialColor::TertiaryContainer, palette.tertiary, 90, 30),
        (MaterialColor::OnTertiaryContainer, palette.tertiary, 30, 90),
        (MaterialColor::ErrorContainer, palette.error, 90, 30),
        (MaterialColor::OnErrorContainer, palette.error, 30, 90),
    ];

    for (role, palette, light_tone, dark_tone) in roles {
        assert_eq!(light.resolve(role), color(palette.tone(light_tone)));
        assert_eq!(dark.resolve(role), color(palette.tone(dark_tone)));
    }
}

#[test]
fn uses_current_dynamic_background_and_surface_tones() {
    let seed = Argb::new(255, 0, 0, 255);
    let palette = CorePalette::of(seed);
    let light = material_colors(Color::new(0, 0, 255), ThemeMode::Light);
    let dark = material_colors(Color::new(0, 0, 255), ThemeMode::Dark);

    for role in [MaterialColor::Background, MaterialColor::Surface] {
        assert_eq!(light.resolve(role), color(palette.neutral.tone(98)));
        assert_eq!(dark.resolve(role), color(palette.neutral.tone(6)));
    }
    for role in [MaterialColor::OnBackground, MaterialColor::OnSurface] {
        assert_eq!(light.resolve(role), color(palette.neutral.tone(10)));
        assert_eq!(dark.resolve(role), color(palette.neutral.tone(90)));
    }
}

#[test]
fn semantic_contract_maps_every_material_role() {
    let seed = Color::new(0, 0, 255);
    let material = material_colors(seed, ThemeMode::Light);
    let semantic = semantic_colors(seed, ThemeMode::Light);
    let mappings = [
        (semantic.primary, MaterialColor::Primary),
        (semantic.on_primary, MaterialColor::OnPrimary),
        (semantic.primary_container, MaterialColor::PrimaryContainer),
        (
            semantic.on_primary_container,
            MaterialColor::OnPrimaryContainer,
        ),
        (semantic.secondary, MaterialColor::Secondary),
        (semantic.on_secondary, MaterialColor::OnSecondary),
        (
            semantic.secondary_container,
            MaterialColor::SecondaryContainer,
        ),
        (
            semantic.on_secondary_container,
            MaterialColor::OnSecondaryContainer,
        ),
        (semantic.tertiary, MaterialColor::Tertiary),
        (semantic.on_tertiary, MaterialColor::OnTertiary),
        (
            semantic.tertiary_container,
            MaterialColor::TertiaryContainer,
        ),
        (
            semantic.on_tertiary_container,
            MaterialColor::OnTertiaryContainer,
        ),
        (semantic.background, MaterialColor::Background),
        (semantic.on_background, MaterialColor::OnBackground),
        (semantic.surface, MaterialColor::Surface),
        (semantic.on_surface, MaterialColor::OnSurface),
        (semantic.error, MaterialColor::Error),
        (semantic.on_error, MaterialColor::OnError),
        (semantic.error_container, MaterialColor::ErrorContainer),
        (semantic.on_error_container, MaterialColor::OnErrorContainer),
    ];

    for (semantic, role) in mappings {
        assert_eq!(semantic, material.resolve(role));
    }
}
