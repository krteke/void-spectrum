use spectrum_core::Length;
use spectrum_schema::{LengthValue, ThemeSpec};
use std::collections::BTreeMap;

use crate::ResolveError;

/// Resolves direct lengths and recursive token references.
pub fn resolve_lengths(spec: &ThemeSpec) -> Result<BTreeMap<String, Length>, ResolveError> {
    let mut resolved = BTreeMap::new();
    for path in spec.lengths.keys() {
        let mut chain = Vec::new();
        resolve_length(path, &spec.lengths, &mut resolved, &mut chain)?;
    }
    Ok(resolved)
}

fn resolve_length(
    path: &str,
    values: &BTreeMap<String, LengthValue>,
    resolved: &mut BTreeMap<String, Length>,
    chain: &mut Vec<String>,
) -> Result<Length, ResolveError> {
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
        .expect("path originates from theme lengths");
    let length = match value {
        LengthValue::Literal(value) => *value,
        LengthValue::Reference(reference) => {
            if !values.contains_key(reference.path()) {
                return Err(ResolveError::UnresolvedReference {
                    token: path.to_owned(),
                    reference: reference.path().to_owned(),
                });
            }
            resolve_length(reference.path(), values, resolved, chain)?
        }
    };
    chain.pop();
    resolved.insert(path.to_owned(), length);
    Ok(length)
}
