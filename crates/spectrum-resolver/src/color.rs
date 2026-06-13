use std::collections::BTreeMap;

use spectrum_core::Color;
use spectrum_schema::{ColorValue, ThemeSpec};

use crate::ResolveError;

/// Resolves direct colors and references to direct colors.
///
/// ```
/// use spectrum_core::Color;
/// use spectrum_resolver::resolve_colors;
/// use spectrum_schema::{ColorValue, ThemeSpec};
///
/// let spec = ThemeSpec::new("Demo")
///     .with_color("accent", ColorValue::Literal(Color::new(80, 120, 200)))
///     .with_color("focus", "{accent}".parse()?);
/// let colors = resolve_colors(&spec)?;
/// assert_eq!(colors["focus"], Color::new(80, 120, 200));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn resolve_colors(spec: &ThemeSpec) -> Result<BTreeMap<String, Color>, ResolveError> {
    let mut resolved = spec
        .colors
        .iter()
        .filter_map(|(path, value)| match value {
            ColorValue::Literal(color) => Some((path.clone(), *color)),
            ColorValue::Reference(_) => None,
        })
        .collect::<BTreeMap<_, _>>();

    for (path, value) in &spec.colors {
        let ColorValue::Reference(reference) = value else {
            continue;
        };
        let color = resolved.get(reference.path()).copied().ok_or_else(|| {
            ResolveError::UnresolvedReference {
                token: path.clone(),
                reference: reference.path().to_owned(),
            }
        })?;
        resolved.insert(path.clone(), color);
    }

    Ok(resolved)
}
