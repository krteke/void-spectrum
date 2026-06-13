//! Tests for theme metadata configuration.

use spectrum_schema::{ThemeMeta, ThemeMode};

#[test]
fn constructor_uses_dark_mode_and_empty_optional_fields() {
    let meta = ThemeMeta::new("Midnight");

    assert_eq!(meta.name, "Midnight");
    assert_eq!(meta.mode, ThemeMode::Dark);
    assert_eq!(meta.author, None);
    assert_eq!(meta.version, None);
    assert_eq!(meta.description, None);
}

#[test]
fn mode_display_uses_configuration_spelling() {
    assert_eq!(ThemeMode::Dark.to_string(), "dark");
    assert_eq!(ThemeMode::Light.to_string(), "light");
}

#[cfg(feature = "json")]
#[test]
fn json_deserializes_metadata_and_defaults_mode() {
    let meta: ThemeMeta =
        serde_json::from_str(r#"{"name":"Dawn","author":"Ada"}"#).expect("valid metadata");

    assert_eq!(meta.name, "Dawn");
    assert_eq!(meta.author.as_deref(), Some("Ada"));
    assert_eq!(meta.mode, ThemeMode::Dark);
}

#[cfg(feature = "json")]
#[test]
fn json_rejects_unknown_fields() {
    let result = serde_json::from_str::<ThemeMeta>(r#"{"name":"Dawn","variant":"light"}"#);

    assert!(result.is_err());
}

#[cfg(feature = "toml")]
#[test]
fn toml_round_trips_light_metadata() {
    let source = "name = \"Dawn\"\nmode = \"light\"\ndescription = \"Bright\"\n";
    let meta: ThemeMeta = toml::from_str(source).expect("valid metadata");
    let encoded = toml::to_string(&meta).expect("serializable metadata");

    assert_eq!(meta.mode, ThemeMode::Light);
    assert!(encoded.contains("mode = \"light\""));
}
