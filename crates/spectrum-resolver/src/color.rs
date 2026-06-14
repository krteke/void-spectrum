use std::collections::BTreeMap;

use spectrum_core::Color;
use spectrum_palette::{MaterialColor, MaterialColors};
use spectrum_schema::{ColorValue, ThemeSpec};

use crate::ResolveError;

/// A fixed color or a binding to a dynamically generated Material role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorBinding {
    /// A concrete color that does not change with the Seed.
    Color(Color),
    /// A Material role resolved against the current generated palette.
    Material(MaterialColor),
}

impl ColorBinding {
    /// Resolves this binding against generated Material colors.
    #[must_use]
    pub const fn resolve(self, material: MaterialColors) -> Color {
        match self {
            Self::Color(color) => color,
            Self::Material(role) => material.resolve(role),
        }
    }
}

/// Resolves direct colors and recursive token references.
///
/// ```
/// use spectrum_core::Color;
/// use spectrum_resolver::{ColorBinding, resolve_colors};
/// use spectrum_schema::{ColorValue, ThemeSpec};
///
/// let spec = ThemeSpec::new("Demo")
///     .with_color("accent", ColorValue::Literal(Color::new(80, 120, 200)))
///     .with_color("focus", "{accent}".parse()?);
/// let colors = resolve_colors(&spec)?;
/// assert_eq!(colors["focus"], ColorBinding::Color(Color::new(80, 120, 200)));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn resolve_colors(spec: &ThemeSpec) -> Result<BTreeMap<String, ColorBinding>, ResolveError> {
    let mut resolved = BTreeMap::new();
    for path in spec.colors.keys() {
        let mut chain = Vec::new();
        resolve_color(path, &spec.colors, &mut resolved, &mut chain)?;
    }
    Ok(resolved)
}

fn resolve_color(
    path: &str,
    values: &BTreeMap<String, ColorValue>,
    resolved: &mut BTreeMap<String, ColorBinding>,
    chain: &mut Vec<String>,
) -> Result<ColorBinding, ResolveError> {
    if let Some(color) = resolved.get(path) {
        return Ok(*color);
    }
    if let Some(start) = chain.iter().position(|entry| entry == path) {
        let mut cycle = chain[start..].to_vec();
        cycle.push(path.to_owned());
        return Err(ResolveError::CircularReference { chain: cycle });
    }

    chain.push(path.to_owned());
    let value = values.get(path).expect("path originates from theme colors");
    let color = match value {
        ColorValue::Literal(color) => ColorBinding::Color(*color),
        ColorValue::Reference(reference) => {
            if let Some(name) = reference.path().strip_prefix("material.") {
                let role = MaterialColor::from_name(name).ok_or_else(|| {
                    ResolveError::UnresolvedReference {
                        token: path.to_owned(),
                        reference: reference.path().to_owned(),
                    }
                })?;
                let color = ColorBinding::Material(role);
                chain.pop();
                resolved.insert(path.to_owned(), color);
                return Ok(color);
            }
            if !values.contains_key(reference.path()) {
                return Err(ResolveError::UnresolvedReference {
                    token: path.to_owned(),
                    reference: reference.path().to_owned(),
                });
            }
            resolve_color(reference.path(), values, resolved, chain)?
        }
    };
    chain.pop();
    resolved.insert(path.to_owned(), color);
    Ok(color)
}
