use std::collections::BTreeSet;

use spectrum_core::ShadowLayer;
use spectrum_schema::ThemeSpec;

use crate::ResolveError;

/// Resolves ordered shadow specifications into validated layers.
pub fn resolve_shadows(spec: &ThemeSpec) -> Result<Vec<(String, ShadowLayer)>, ResolveError> {
    let mut paths = BTreeSet::new();
    spec.shadows
        .iter()
        .map(|shadow| {
            if !paths.insert(shadow.path.clone()) {
                return Err(ResolveError::DuplicateShadow {
                    token: shadow.path.clone(),
                });
            }
            let layer = ShadowLayer::new(
                shadow.color,
                shadow.offset_x,
                shadow.offset_y,
                shadow.blur,
                shadow.spread,
            )
            .map_err(|source| ResolveError::InvalidShadow {
                token: shadow.path.clone(),
                source,
            })?;
            Ok((shadow.path.clone(), layer))
        })
        .collect()
}
