//! Tests for the top-level theme specification.

use spectrum_core::Color;
use spectrum_schema::ThemeSpec;

#[test]
fn constructor_has_no_seed() {
    let spec = ThemeSpec::new("Midnight");

    assert_eq!(spec.meta.name, "Midnight");
    assert_eq!(spec.seed, None);
}

#[test]
fn builder_sets_a_typed_seed() {
    let seed = Color::new_rgba(80, 120, 200, 128);
    let spec = ThemeSpec::new("Midnight").with_seed(seed);

    assert_eq!(spec.seed, Some(seed));
}

#[cfg(feature = "json")]
#[test]
fn json_uses_hex_seed_representation() {
    let source = r##"{"meta":{"name":"Dawn","mode":"light"},"seed":"#5078c8"}"##;
    let spec: ThemeSpec = serde_json::from_str(source).expect("valid specification");
    let encoded = serde_json::to_string(&spec).expect("serializable specification");

    assert_eq!(spec.seed, Some(Color::new(80, 120, 200)));
    assert!(encoded.contains(r##""seed":"#5078c8""##));
}

#[cfg(feature = "json")]
#[test]
fn json_rejects_invalid_seed_colors() {
    let source = r#"{"meta":{"name":"Dawn"},"seed":"5078c8"}"#;

    assert!(serde_json::from_str::<ThemeSpec>(source).is_err());
}

#[cfg(feature = "toml")]
#[test]
fn toml_supports_an_optional_rgba_seed() {
    let source = "seed = \"#5078c880\"\n\n[meta]\nname = \"Dawn\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.seed, Some(Color::new_rgba(80, 120, 200, 128)));
}
