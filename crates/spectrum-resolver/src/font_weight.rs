use spectrum_core::FontWeight;
use spectrum_schema::{FontWeightValue, ThemeSpec};
use std::collections::BTreeMap;

use crate::ResolveError;

/// Resolves direct font weights and recursive token references.
pub fn resolve_font_weights(
    spec: &ThemeSpec,
) -> Result<BTreeMap<String, FontWeight>, ResolveError> {
    let mut resolved = BTreeMap::new();
    for path in spec.font_weights.keys() {
        let mut chain = Vec::new();
        resolve_font_weight(path, &spec.font_weights, &mut resolved, &mut chain)?;
    }
    Ok(resolved)
}

fn resolve_font_weight(
    path: &str,
    values: &BTreeMap<String, FontWeightValue>,
    resolved: &mut BTreeMap<String, FontWeight>,
    chain: &mut Vec<String>,
) -> Result<FontWeight, ResolveError> {
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
        .expect("path originates from theme font weights");
    let weight = match value {
        FontWeightValue::Literal(value) => *value,
        FontWeightValue::Reference(reference) => {
            if !values.contains_key(reference.path()) {
                return Err(ResolveError::UnresolvedReference {
                    token: path.to_owned(),
                    reference: reference.path().to_owned(),
                });
            }
            resolve_font_weight(reference.path(), values, resolved, chain)?
        }
    };
    chain.pop();
    resolved.insert(path.to_owned(), weight);
    Ok(weight)
}
