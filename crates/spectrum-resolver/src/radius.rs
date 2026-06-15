use spectrum_core::Radius;
use spectrum_schema::{RadiusValue, ThemeSpec};
use std::collections::BTreeMap;

use crate::ResolveError;

/// Resolves direct radii and recursive token references.
pub fn resolve_radii(spec: &ThemeSpec) -> Result<BTreeMap<String, Radius>, ResolveError> {
    let mut resolved = BTreeMap::new();
    for path in spec.radii.keys() {
        let mut chain = Vec::new();
        resolve_radius(path, &spec.radii, &mut resolved, &mut chain)?;
    }
    Ok(resolved)
}

fn resolve_radius(
    path: &str,
    values: &BTreeMap<String, RadiusValue>,
    resolved: &mut BTreeMap<String, Radius>,
    chain: &mut Vec<String>,
) -> Result<Radius, ResolveError> {
    if let Some(value) = resolved.get(path) {
        return Ok(*value);
    }
    if let Some(start) = chain.iter().position(|entry| entry == path) {
        let mut cycle = chain[start..].to_vec();
        cycle.push(path.to_owned());
        return Err(ResolveError::CircularReference { chain: cycle });
    }

    chain.push(path.to_owned());
    let value = values.get(path).expect("path originates from theme radii");
    let radius = match value {
        RadiusValue::Literal(value) => *value,
        RadiusValue::Reference(reference) => {
            if !values.contains_key(reference.path()) {
                return Err(ResolveError::UnresolvedReference {
                    token: path.to_owned(),
                    reference: reference.path().to_owned(),
                });
            }
            resolve_radius(reference.path(), values, resolved, chain)?
        }
    };
    chain.pop();
    resolved.insert(path.to_owned(), radius);
    Ok(radius)
}
