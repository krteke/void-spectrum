use spectrum_core::{Color, SemanticColors, ThemeMode};

use crate::domains::ColorDomains;

/// Derives core semantic colors from a seed and target surface mode.
#[must_use]
pub fn semantic_colors(seed: Color, mode: ThemeMode) -> SemanticColors {
    let domains = ColorDomains::from_seed(seed);
    let (primary, on_primary, background, on_background, error, on_error) = match mode {
        ThemeMode::Light => (40, 100, 99, 10, 40, 100),
        ThemeMode::Dark => (80, 20, 10, 90, 80, 20),
    };

    SemanticColors {
        primary: domains.primary.tone(primary),
        on_primary: domains.primary.tone(on_primary),
        background: domains.neutral.tone(background),
        on_background: domains.neutral.tone(on_background),
        surface: domains.neutral.tone(background),
        on_surface: domains.neutral.tone(on_background),
        error: domains.error.tone(error),
        on_error: domains.error.tone(on_error),
    }
}
