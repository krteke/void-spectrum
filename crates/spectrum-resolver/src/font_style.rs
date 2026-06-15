use spectrum_core::FontStyle;
use spectrum_schema::{FontStyleValue, ThemeSpec};
use std::collections::BTreeMap;

use crate::ResolveError;

/// Resolves direct font styles and recursive token references.
pub fn resolve_font_styles(spec: &ThemeSpec) -> Result<BTreeMap<String, FontStyle>, ResolveError> {
    let mut resolved = BTreeMap::new();
    for path in spec.font_styles.keys() {
        let mut chain = Vec::new();
        resolve_font_style(path, &spec.font_styles, &mut resolved, &mut chain)?;
    }
    Ok(resolved)
}

fn resolve_font_style(
    path: &str,
    values: &BTreeMap<String, FontStyleValue>,
    resolved: &mut BTreeMap<String, FontStyle>,
    chain: &mut Vec<String>,
) -> Result<FontStyle, ResolveError> {
    if let Some(value) = resolved.get(path) {
        return Ok(*value);
    }
    if let Some(start) = chain.iter().position(|entry| entry == path) {
        let mut cycle = chain[start..].to_vec();
        cycle.push(path.to_owned());
        return Err(ResolveError::CircularReference { chain: cycle });
    }

    chain.push(path.to_owned());
    let value = values
        .get(path)
        .expect("path originates from theme font styles");
    let style = match value {
        FontStyleValue::Literal(value) => *value,
        FontStyleValue::Reference(reference) => {
            if !values.contains_key(reference.path()) {
                return Err(ResolveError::UnresolvedReference {
                    token: path.to_owned(),
                    reference: reference.path().to_owned(),
                });
            }
            resolve_font_style(reference.path(), values, resolved, chain)?
        }
    };
    chain.pop();
    resolved.insert(path.to_owned(), style);
    Ok(style)
}
