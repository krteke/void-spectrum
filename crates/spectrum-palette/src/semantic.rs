use spectrum_core::{Color, SemanticColors, ThemeMode};

use crate::{MaterialColor, MaterialColors, domains::ColorDomains};

/// Generates Material color source values from a seed and target surface mode.
#[must_use]
pub fn material_colors(seed: Color, mode: ThemeMode) -> MaterialColors {
    let domains = ColorDomains::from_seed(seed);
    let (primary, on_primary, background, on_background, error, on_error) = match mode {
        ThemeMode::Light => (40, 100, 99, 10, 40, 100),
        ThemeMode::Dark => (80, 20, 10, 90, 80, 20),
    };

    MaterialColors([
        domains.primary.tone(primary),
        domains.primary.tone(on_primary),
        domains.neutral.tone(background),
        domains.neutral.tone(on_background),
        domains.neutral.tone(background),
        domains.neutral.tone(on_background),
        domains.error.tone(error),
        domains.error.tone(on_error),
    ])
}

/// Maps generated Material sources to the built-in semantic color contract.
#[must_use]
pub fn semantic_colors(seed: Color, mode: ThemeMode) -> SemanticColors {
    let colors = material_colors(seed, mode);
    SemanticColors {
        primary: colors.resolve(MaterialColor::Primary),
        on_primary: colors.resolve(MaterialColor::OnPrimary),
        background: colors.resolve(MaterialColor::Background),
        on_background: colors.resolve(MaterialColor::OnBackground),
        surface: colors.resolve(MaterialColor::Surface),
        on_surface: colors.resolve(MaterialColor::OnSurface),
        error: colors.resolve(MaterialColor::Error),
        on_error: colors.resolve(MaterialColor::OnError),
    }
}
