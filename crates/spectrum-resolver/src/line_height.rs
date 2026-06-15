use spectrum_core::LineHeight;
use spectrum_schema::{LineHeightValue, ThemeSpec};
use std::collections::BTreeMap;

use crate::ResolveError;

/// Resolves direct line heights and recursive token references.
pub fn resolve_line_heights(
    spec: &ThemeSpec,
) -> Result<BTreeMap<String, LineHeight>, ResolveError> {
    let mut resolved = BTreeMap::new();
    for path in spec.line_heights.keys() {
        let mut chain = Vec::new();
        resolve_line_height(path, &spec.line_heights, &mut resolved, &mut chain)?;
    }
    Ok(resolved)
}

fn resolve_line_height(
    path: &str,
    values: &BTreeMap<String, LineHeightValue>,
    resolved: &mut BTreeMap<String, LineHeight>,
    chain: &mut Vec<String>,
) -> Result<LineHeight, ResolveError> {
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
        .expect("path originates from theme line heights");
    let line_height = match value {
        LineHeightValue::Literal(value) => *value,
        LineHeightValue::Reference(reference) => {
            if !values.contains_key(reference.path()) {
                return Err(ResolveError::UnresolvedReference {
                    token: path.to_owned(),
                    reference: reference.path().to_owned(),
                });
            }
            resolve_line_height(reference.path(), values, resolved, chain)?
        }
    };
    chain.pop();
    resolved.insert(path.to_owned(), line_height);
    Ok(line_height)
}
