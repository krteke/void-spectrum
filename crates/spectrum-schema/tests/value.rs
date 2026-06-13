//! Tests for color token override values.

use spectrum_core::Color;
use spectrum_schema::{ColorValue, ColorValueParseError, ThemeSpec};

#[test]
fn parses_literal_colors_and_token_references() {
    assert_eq!(
        "#5078c8".parse(),
        Ok(ColorValue::Literal(Color::new(80, 120, 200)))
    );

    let reference = "{surface.panel}"
        .parse::<ColorValue>()
        .expect("valid reference");
    assert_eq!(reference.to_string(), "{surface.panel}");
}

#[test]
fn rejects_invalid_reference_paths() {
    for input in ["{}", "{surface..panel}", "{surface panel}"] {
        assert_eq!(
            input.parse::<ColorValue>(),
            Err(ColorValueParseError::InvalidReference)
        );
    }
}

#[test]
fn builder_adds_color_overrides() {
    let spec = ThemeSpec::new("Midnight").with_color(
        "text.primary",
        ColorValue::Literal(Color::new(240, 240, 240)),
    );

    assert!(spec.colors.contains_key("text.primary"));
}

#[cfg(feature = "json")]
#[test]
fn json_decodes_literal_and_reference_values() {
    let source = r##"{"meta":{"name":"Dawn"},"colors":{"text.primary":"#102030","border.focus":"{text.primary}"}}"##;
    let spec: ThemeSpec = serde_json::from_str(source).expect("valid specification");

    assert_eq!(spec.colors["text.primary"].to_string(), "#102030");
    assert_eq!(spec.colors["border.focus"].to_string(), "{text.primary}");
}

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_color_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[colors]\n\"text.primary\" = \"#102030\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.colors["text.primary"].to_string(), "#102030");
}
