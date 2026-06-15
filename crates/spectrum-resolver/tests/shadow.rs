//! Tests for ordered shadow layer resolution.

use spectrum_core::{Color, Length, LengthUnit, ShadowError};
use spectrum_resolver::{ResolveError, resolve_shadows};
use spectrum_schema::{ShadowSpec, ThemeSpec};

fn shadow(path: &str, blur: f32) -> ShadowSpec {
    let px = |value| Length::new(value, LengthUnit::Px).expect("finite");
    ShadowSpec {
        path: path.to_owned(),
        color: Color::new_rgba(0, 0, 0, 64),
        offset_x: px(0.0),
        offset_y: px(2.0),
        blur: px(blur),
        spread: px(0.0),
    }
}

#[test]
fn resolves_layers_in_configuration_order() {
    let spec = ThemeSpec::new("Demo")
        .with_shadow(shadow("shadow.card", 4.0))
        .with_shadow(shadow("shadow.dialog", 24.0));

    let resolved = resolve_shadows(&spec).expect("resolved shadows");

    assert_eq!(resolved[0].0, "shadow.card");
    assert_eq!(resolved[1].0, "shadow.dialog");
    assert_eq!(resolved[1].1.blur(), "24px".parse().expect("length"));
}

#[test]
fn rejects_duplicate_paths() {
    let spec = ThemeSpec::new("Demo")
        .with_shadow(shadow("shadow.card", 4.0))
        .with_shadow(shadow("shadow.card", 8.0));

    assert_eq!(
        resolve_shadows(&spec),
        Err(ResolveError::DuplicateShadow {
            token: "shadow.card".to_owned()
        })
    );
}

#[test]
fn rejects_negative_blur() {
    let spec = ThemeSpec::new("Demo").with_shadow(shadow("shadow.card", -1.0));

    assert_eq!(
        resolve_shadows(&spec),
        Err(ResolveError::InvalidShadow {
            token: "shadow.card".to_owned(),
            source: ShadowError::NegativeBlur,
        })
    );
}
