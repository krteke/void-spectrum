use spectrum_core::{Color, SemanticColors, ThemeMode};

use crate::{MaterialColor, MaterialColors, domains::ColorDomains};

/// Generates Material color source values from a seed and target surface mode.
#[must_use]
pub fn material_colors(seed: Color, mode: ThemeMode) -> MaterialColors {
    let domains = ColorDomains::from_seed(seed);
    let (accent, on_accent, container, on_container, background, on_background) = match mode {
        ThemeMode::Light => (40, 100, 90, 30, 98, 10),
        ThemeMode::Dark => (80, 20, 30, 90, 6, 90),
    };

    MaterialColors([
        domains.primary.tone(accent),
        domains.primary.tone(on_accent),
        domains.primary.tone(container),
        domains.primary.tone(on_container),
        domains.secondary.tone(accent),
        domains.secondary.tone(on_accent),
        domains.secondary.tone(container),
        domains.secondary.tone(on_container),
        domains.tertiary.tone(accent),
        domains.tertiary.tone(on_accent),
        domains.tertiary.tone(container),
        domains.tertiary.tone(on_container),
        domains.neutral.tone(background),
        domains.neutral.tone(on_background),
        domains.neutral.tone(background),
        domains.neutral.tone(on_background),
        domains.error.tone(accent),
        domains.error.tone(on_accent),
        domains.error.tone(container),
        domains.error.tone(on_container),
    ])
}

/// Maps generated Material sources to the built-in semantic color contract.
#[must_use]
pub fn semantic_colors(seed: Color, mode: ThemeMode) -> SemanticColors {
    let colors = material_colors(seed, mode);
    SemanticColors {
        primary: colors.resolve(MaterialColor::Primary),
        on_primary: colors.resolve(MaterialColor::OnPrimary),
        primary_container: colors.resolve(MaterialColor::PrimaryContainer),
        on_primary_container: colors.resolve(MaterialColor::OnPrimaryContainer),
        secondary: colors.resolve(MaterialColor::Secondary),
        on_secondary: colors.resolve(MaterialColor::OnSecondary),
        secondary_container: colors.resolve(MaterialColor::SecondaryContainer),
        on_secondary_container: colors.resolve(MaterialColor::OnSecondaryContainer),
        tertiary: colors.resolve(MaterialColor::Tertiary),
        on_tertiary: colors.resolve(MaterialColor::OnTertiary),
        tertiary_container: colors.resolve(MaterialColor::TertiaryContainer),
        on_tertiary_container: colors.resolve(MaterialColor::OnTertiaryContainer),
        background: colors.resolve(MaterialColor::Background),
        on_background: colors.resolve(MaterialColor::OnBackground),
        surface: colors.resolve(MaterialColor::Surface),
        on_surface: colors.resolve(MaterialColor::OnSurface),
        error: colors.resolve(MaterialColor::Error),
        on_error: colors.resolve(MaterialColor::OnError),
        error_container: colors.resolve(MaterialColor::ErrorContainer),
        on_error_container: colors.resolve(MaterialColor::OnErrorContainer),
    }
}
