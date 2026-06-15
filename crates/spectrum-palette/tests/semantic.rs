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
    assert_eq!(
        MaterialColor::from_name("surface_container_highest"),
        Some(MaterialColor::SurfaceContainerHighest)
    );
    assert_eq!(
        MaterialColor::from_name("on_tertiary_fixed_variant"),
        Some(MaterialColor::OnTertiaryFixedVariant)
    );
    assert_eq!(MaterialColor::from_name("unknown"), None);
}

#[test]
fn every_material_role_name_round_trips() {
    let colors = material_colors(Color::new(0, 0, 255), ThemeMode::Light);

    for role in [
        MaterialColor::Primary,
        MaterialColor::OnPrimary,
        MaterialColor::PrimaryContainer,
        MaterialColor::OnPrimaryContainer,
        MaterialColor::Secondary,
        MaterialColor::OnSecondary,
        MaterialColor::SecondaryContainer,
        MaterialColor::OnSecondaryContainer,
        MaterialColor::Tertiary,
        MaterialColor::OnTertiary,
        MaterialColor::TertiaryContainer,
        MaterialColor::OnTertiaryContainer,
        MaterialColor::Background,
        MaterialColor::OnBackground,
        MaterialColor::Surface,
        MaterialColor::OnSurface,
        MaterialColor::SurfaceDim,
        MaterialColor::SurfaceBright,
        MaterialColor::SurfaceContainerLowest,
        MaterialColor::SurfaceContainerLow,
        MaterialColor::SurfaceContainer,
        MaterialColor::SurfaceContainerHigh,
        MaterialColor::SurfaceContainerHighest,
        MaterialColor::SurfaceVariant,
        MaterialColor::OnSurfaceVariant,
        MaterialColor::InverseSurface,
        MaterialColor::InverseOnSurface,
        MaterialColor::Outline,
        MaterialColor::OutlineVariant,
        MaterialColor::Shadow,
        MaterialColor::Scrim,
        MaterialColor::SurfaceTint,
        MaterialColor::InversePrimary,
        MaterialColor::PrimaryFixed,
        MaterialColor::PrimaryFixedDim,
        MaterialColor::OnPrimaryFixed,
        MaterialColor::OnPrimaryFixedVariant,
        MaterialColor::SecondaryFixed,
        MaterialColor::SecondaryFixedDim,
        MaterialColor::OnSecondaryFixed,
        MaterialColor::OnSecondaryFixedVariant,
        MaterialColor::TertiaryFixed,
        MaterialColor::TertiaryFixedDim,
        MaterialColor::OnTertiaryFixed,
        MaterialColor::OnTertiaryFixedVariant,
        MaterialColor::Error,
        MaterialColor::OnError,
        MaterialColor::ErrorContainer,
        MaterialColor::OnErrorContainer,
    ] {
        assert_eq!(MaterialColor::from_name(role.name()), Some(role));
        let _ = colors.resolve(role);
    }
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
fn maps_surface_outline_and_inverse_roles() {
    let seed = Argb::new(255, 0, 0, 255);
    let palette = CorePalette::of(seed);
    let light = material_colors(Color::new(0, 0, 255), ThemeMode::Light);
    let dark = material_colors(Color::new(0, 0, 255), ThemeMode::Dark);
    let roles = [
        (MaterialColor::SurfaceDim, palette.neutral, 87, 6),
        (MaterialColor::SurfaceBright, palette.neutral, 98, 24),
        (
            MaterialColor::SurfaceContainerLowest,
            palette.neutral,
            100,
            4,
        ),
        (MaterialColor::SurfaceContainerLow, palette.neutral, 96, 10),
        (MaterialColor::SurfaceContainer, palette.neutral, 94, 12),
        (MaterialColor::SurfaceContainerHigh, palette.neutral, 92, 17),
        (
            MaterialColor::SurfaceContainerHighest,
            palette.neutral,
            90,
            22,
        ),
        (
            MaterialColor::SurfaceVariant,
            palette.neutral_variant,
            90,
            30,
        ),
        (
            MaterialColor::OnSurfaceVariant,
            palette.neutral_variant,
            30,
            80,
        ),
        (MaterialColor::InverseSurface, palette.neutral, 20, 90),
        (MaterialColor::InverseOnSurface, palette.neutral, 95, 20),
        (MaterialColor::Outline, palette.neutral_variant, 50, 60),
        (
            MaterialColor::OutlineVariant,
            palette.neutral_variant,
            80,
            30,
        ),
        (MaterialColor::InversePrimary, palette.primary, 80, 40),
    ];

    for (role, palette, light_tone, dark_tone) in roles {
        assert_eq!(light.resolve(role), color(palette.tone(light_tone)));
        assert_eq!(dark.resolve(role), color(palette.tone(dark_tone)));
    }
    for role in [MaterialColor::Shadow, MaterialColor::Scrim] {
        assert_eq!(light.resolve(role), color(palette.neutral.tone(0)));
        assert_eq!(dark.resolve(role), color(palette.neutral.tone(0)));
    }
    assert_eq!(
        light.resolve(MaterialColor::SurfaceTint),
        color(palette.primary.tone(40))
    );
    assert_eq!(
        dark.resolve(MaterialColor::SurfaceTint),
        color(palette.primary.tone(80))
    );
}

#[test]
fn maps_all_fixed_roles_independently_of_mode() {
    let seed = Argb::new(255, 0, 0, 255);
    let palette = CorePalette::of(seed);
    let light = material_colors(Color::new(0, 0, 255), ThemeMode::Light);
    let dark = material_colors(Color::new(0, 0, 255), ThemeMode::Dark);
    let roles = [
        (MaterialColor::PrimaryFixed, palette.primary, 90),
        (MaterialColor::PrimaryFixedDim, palette.primary, 80),
        (MaterialColor::OnPrimaryFixed, palette.primary, 10),
        (MaterialColor::OnPrimaryFixedVariant, palette.primary, 30),
        (MaterialColor::SecondaryFixed, palette.secondary, 90),
        (MaterialColor::SecondaryFixedDim, palette.secondary, 80),
        (MaterialColor::OnSecondaryFixed, palette.secondary, 10),
        (
            MaterialColor::OnSecondaryFixedVariant,
            palette.secondary,
            30,
        ),
        (MaterialColor::TertiaryFixed, palette.tertiary, 90),
        (MaterialColor::TertiaryFixedDim, palette.tertiary, 80),
        (MaterialColor::OnTertiaryFixed, palette.tertiary, 10),
        (MaterialColor::OnTertiaryFixedVariant, palette.tertiary, 30),
    ];

    for (role, palette, tone) in roles {
        let expected = color(palette.tone(tone));
        assert_eq!(light.resolve(role), expected);
        assert_eq!(dark.resolve(role), expected);
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
        (semantic.surface_dim, MaterialColor::SurfaceDim),
        (semantic.surface_bright, MaterialColor::SurfaceBright),
        (
            semantic.surface_container_lowest,
            MaterialColor::SurfaceContainerLowest,
        ),
        (
            semantic.surface_container_low,
            MaterialColor::SurfaceContainerLow,
        ),
        (semantic.surface_container, MaterialColor::SurfaceContainer),
        (
            semantic.surface_container_high,
            MaterialColor::SurfaceContainerHigh,
        ),
        (
            semantic.surface_container_highest,
            MaterialColor::SurfaceContainerHighest,
        ),
        (semantic.surface_variant, MaterialColor::SurfaceVariant),
        (semantic.on_surface_variant, MaterialColor::OnSurfaceVariant),
        (semantic.inverse_surface, MaterialColor::InverseSurface),
        (semantic.inverse_on_surface, MaterialColor::InverseOnSurface),
        (semantic.outline, MaterialColor::Outline),
        (semantic.outline_variant, MaterialColor::OutlineVariant),
        (semantic.shadow, MaterialColor::Shadow),
        (semantic.scrim, MaterialColor::Scrim),
        (semantic.surface_tint, MaterialColor::SurfaceTint),
        (semantic.inverse_primary, MaterialColor::InversePrimary),
        (semantic.primary_fixed, MaterialColor::PrimaryFixed),
        (semantic.primary_fixed_dim, MaterialColor::PrimaryFixedDim),
        (semantic.on_primary_fixed, MaterialColor::OnPrimaryFixed),
        (
            semantic.on_primary_fixed_variant,
            MaterialColor::OnPrimaryFixedVariant,
        ),
        (semantic.secondary_fixed, MaterialColor::SecondaryFixed),
        (
            semantic.secondary_fixed_dim,
            MaterialColor::SecondaryFixedDim,
        ),
        (semantic.on_secondary_fixed, MaterialColor::OnSecondaryFixed),
        (
            semantic.on_secondary_fixed_variant,
            MaterialColor::OnSecondaryFixedVariant,
        ),
        (semantic.tertiary_fixed, MaterialColor::TertiaryFixed),
        (semantic.tertiary_fixed_dim, MaterialColor::TertiaryFixedDim),
        (semantic.on_tertiary_fixed, MaterialColor::OnTertiaryFixed),
        (
            semantic.on_tertiary_fixed_variant,
            MaterialColor::OnTertiaryFixedVariant,
        ),
        (semantic.error, MaterialColor::Error),
        (semantic.on_error, MaterialColor::OnError),
        (semantic.error_container, MaterialColor::ErrorContainer),
        (semantic.on_error_container, MaterialColor::OnErrorContainer),
    ];

    for (semantic, role) in mappings {
        assert_eq!(semantic, material.resolve(role));
    }
}
