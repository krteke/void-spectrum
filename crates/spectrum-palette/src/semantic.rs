use spectrum_core::{Color, SemanticColors, ThemeMode};

use crate::{MaterialColors, domains::ColorDomains, material::MaterialToneSet};

/// Generates Material color source values from a seed and target surface mode.
#[must_use]
pub fn material_colors(seed: Color, mode: ThemeMode) -> MaterialColors {
    let domains = ColorDomains::from_seed(seed);
    let tones = MaterialToneSet::for_mode(mode);

    MaterialColors {
        primary: domains.primary.tone(tones.accent),
        on_primary: domains.primary.tone(tones.on_accent),
        primary_container: domains.primary.tone(tones.container),
        on_primary_container: domains.primary.tone(tones.on_container),

        secondary: domains.secondary.tone(tones.accent),
        on_secondary: domains.secondary.tone(tones.on_accent),
        secondary_container: domains.secondary.tone(tones.container),
        on_secondary_container: domains.secondary.tone(tones.on_container),

        tertiary: domains.tertiary.tone(tones.accent),
        on_tertiary: domains.tertiary.tone(tones.on_accent),
        tertiary_container: domains.tertiary.tone(tones.container),
        on_tertiary_container: domains.tertiary.tone(tones.on_container),

        background: domains.neutral.tone(tones.background),
        on_background: domains.neutral.tone(tones.on_background),
        surface: domains.neutral.tone(tones.background),
        on_surface: domains.neutral.tone(tones.on_background),

        surface_dim: domains.neutral.tone(tones.surface_dim),
        surface_bright: domains.neutral.tone(tones.surface_bright),
        surface_container_lowest: domains.neutral.tone(tones.surface_container_lowest),
        surface_container_low: domains.neutral.tone(tones.surface_container_low),
        surface_container: domains.neutral.tone(tones.surface_container),
        surface_container_high: domains.neutral.tone(tones.surface_container_high),
        surface_container_highest: domains.neutral.tone(tones.surface_container_highest),

        surface_variant: domains.neutral_variant.tone(tones.surface_variant),
        on_surface_variant: domains.neutral_variant.tone(tones.on_surface_variant),

        inverse_surface: domains.neutral.tone(tones.inverse_surface),
        inverse_on_surface: domains.neutral.tone(tones.inverse_on_surface),

        outline: domains.neutral_variant.tone(tones.outline),
        outline_variant: domains.neutral_variant.tone(tones.outline_variant),

        shadow: domains.neutral.tone(0),
        scrim: domains.neutral.tone(0),

        surface_tint: domains.primary.tone(tones.accent),
        inverse_primary: domains.primary.tone(tones.inverse_primary),

        primary_fixed: domains.primary.tone(90),
        primary_fixed_dim: domains.primary.tone(80),
        on_primary_fixed: domains.primary.tone(10),
        on_primary_fixed_variant: domains.primary.tone(30),

        secondary_fixed: domains.secondary.tone(90),
        secondary_fixed_dim: domains.secondary.tone(80),
        on_secondary_fixed: domains.secondary.tone(10),
        on_secondary_fixed_variant: domains.secondary.tone(30),

        tertiary_fixed: domains.tertiary.tone(90),
        tertiary_fixed_dim: domains.tertiary.tone(80),
        on_tertiary_fixed: domains.tertiary.tone(10),
        on_tertiary_fixed_variant: domains.tertiary.tone(30),

        error: domains.error.tone(tones.accent),
        on_error: domains.error.tone(tones.on_accent),
        error_container: domains.error.tone(tones.container),
        on_error_container: domains.error.tone(tones.on_container),
    }
}

/// Maps generated Material sources to the built-in semantic color contract.
#[must_use]
pub fn semantic_colors(seed: Color, mode: ThemeMode) -> SemanticColors {
    material_colors(seed, mode).into()
}

impl From<MaterialColors> for SemanticColors {
    fn from(colors: MaterialColors) -> Self {
        Self {
            primary: colors.primary,
            on_primary: colors.on_primary,
            primary_container: colors.primary_container,
            on_primary_container: colors.on_primary_container,
            secondary: colors.secondary,
            on_secondary: colors.on_secondary,
            secondary_container: colors.secondary_container,
            on_secondary_container: colors.on_secondary_container,
            tertiary: colors.tertiary,
            on_tertiary: colors.on_tertiary,
            tertiary_container: colors.tertiary_container,
            on_tertiary_container: colors.on_tertiary_container,
            background: colors.background,
            on_background: colors.on_background,
            surface: colors.surface,
            on_surface: colors.on_surface,
            surface_dim: colors.surface_dim,
            surface_bright: colors.surface_bright,
            surface_container_lowest: colors.surface_container_lowest,
            surface_container_low: colors.surface_container_low,
            surface_container: colors.surface_container,
            surface_container_high: colors.surface_container_high,
            surface_container_highest: colors.surface_container_highest,
            surface_variant: colors.surface_variant,
            on_surface_variant: colors.on_surface_variant,
            inverse_surface: colors.inverse_surface,
            inverse_on_surface: colors.inverse_on_surface,
            outline: colors.outline,
            outline_variant: colors.outline_variant,
            shadow: colors.shadow,
            scrim: colors.scrim,
            surface_tint: colors.surface_tint,
            inverse_primary: colors.inverse_primary,
            primary_fixed: colors.primary_fixed,
            primary_fixed_dim: colors.primary_fixed_dim,
            on_primary_fixed: colors.on_primary_fixed,
            on_primary_fixed_variant: colors.on_primary_fixed_variant,
            secondary_fixed: colors.secondary_fixed,
            secondary_fixed_dim: colors.secondary_fixed_dim,
            on_secondary_fixed: colors.on_secondary_fixed,
            on_secondary_fixed_variant: colors.on_secondary_fixed_variant,
            tertiary_fixed: colors.tertiary_fixed,
            tertiary_fixed_dim: colors.tertiary_fixed_dim,
            on_tertiary_fixed: colors.on_tertiary_fixed,
            on_tertiary_fixed_variant: colors.on_tertiary_fixed_variant,
            error: colors.error,
            on_error: colors.on_error,
            error_container: colors.error_container,
            on_error_container: colors.on_error_container,
        }
    }
}
