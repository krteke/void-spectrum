use std::collections::BTreeMap;

use spectrum_core::Color;
use spectrum_schema::{ThemeMeta, ThemeSpec};

use crate::{ColorBinding, ResolveError, resolve_colors};

/// A theme whose configured color references have been fully resolved.
///
/// ```
/// use spectrum_core::Color;
/// use spectrum_resolver::{ColorBinding, resolve_theme};
/// use spectrum_schema::ThemeSpec;
///
/// let spec = ThemeSpec::new("Demo")
///     .with_seed(Color::new(80, 120, 200))
///     .with_color("accent", "#5078c8".parse()?);
/// let theme = resolve_theme(&spec)?;
/// assert_eq!(theme.meta.name, "Demo");
/// assert_eq!(theme.colors["accent"], ColorBinding::Color(Color::new(80, 120, 200)));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTheme {
    /// Human-readable theme metadata.
    pub meta: ThemeMeta,
    /// Optional source color for future palette derivation.
    pub seed: Option<Color>,
    /// Color bindings with token references fully expanded.
    pub colors: BTreeMap<String, ColorBinding>,
}

/// Resolves a theme specification into an owned theme.
pub fn resolve_theme(spec: &ThemeSpec) -> Result<ResolvedTheme, ResolveError> {
    Ok(ResolvedTheme {
        meta: spec.meta.clone(),
        seed: spec.seed,
        colors: resolve_colors(spec)?,
    })
}
