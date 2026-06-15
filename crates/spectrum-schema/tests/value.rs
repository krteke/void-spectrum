//! Tests for color token override values.

use spectrum_core::{Color, FontWeight, Length, LengthUnit, Radius};
use spectrum_schema::{
    ColorValue, ColorValueParseError, FontWeightValue, FontWeightValueParseError, LengthValue,
    LengthValueParseError, RadiusValue, RadiusValueParseError, ThemeSpec,
};

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
fn parses_length_literals_and_references() {
    assert_eq!(
        "8px".parse(),
        Ok(LengthValue::Literal(
            Length::new(8.0, LengthUnit::Px).expect("finite")
        ))
    );

    let reference = "{spacing.medium}"
        .parse::<LengthValue>()
        .expect("valid reference");
    assert_eq!(reference.to_string(), "{spacing.medium}");
}

#[test]
fn rejects_invalid_length_values() {
    assert!(matches!(
        "8pt".parse::<LengthValue>(),
        Err(LengthValueParseError::InvalidLength(_))
    ));
    assert_eq!(
        "{spacing..medium}".parse::<LengthValue>(),
        Err(LengthValueParseError::InvalidReference)
    );
}

#[test]
fn parses_radius_literals_and_references() {
    let radius = Radius::new(Length::new(8.0, LengthUnit::Px).expect("finite")).expect("radius");
    assert_eq!("8px".parse(), Ok(RadiusValue::Literal(radius)));

    let reference = "{radius.card}"
        .parse::<RadiusValue>()
        .expect("valid reference");
    assert_eq!(reference.to_string(), "{radius.card}");
}

#[test]
fn rejects_invalid_radius_values() {
    for input in ["-1px", "8pt"] {
        assert!(matches!(
            input.parse::<RadiusValue>(),
            Err(RadiusValueParseError::InvalidRadius(_))
        ));
    }
    assert_eq!(
        "{radius..card}".parse::<RadiusValue>(),
        Err(RadiusValueParseError::InvalidReference)
    );
}

#[test]
fn parses_font_weight_literals_and_references() {
    assert_eq!(
        "650".parse(),
        Ok(FontWeightValue::Literal(
            FontWeight::new(650).expect("weight")
        ))
    );

    let reference = "{font.body}"
        .parse::<FontWeightValue>()
        .expect("valid reference");
    assert_eq!(reference.to_string(), "{font.body}");
}

#[test]
fn rejects_invalid_font_weight_values() {
    for input in ["bold", "0", "1001"] {
        assert!(matches!(
            input.parse::<FontWeightValue>(),
            Err(FontWeightValueParseError::InvalidFontWeight(_))
        ));
    }
    assert_eq!(
        "{font..body}".parse::<FontWeightValue>(),
        Err(FontWeightValueParseError::InvalidReference)
    );
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

#[cfg(feature = "json")]
#[test]
fn json_decodes_length_values() {
    let source = r#"{"meta":{"name":"Dawn"},"lengths":{"spacing.small":"4px","spacing.medium":"{spacing.small}"}}"#;
    let spec: ThemeSpec = serde_json::from_str(source).expect("valid specification");

    assert_eq!(spec.lengths["spacing.small"].to_string(), "4px");
    assert_eq!(
        spec.lengths["spacing.medium"].to_string(),
        "{spacing.small}"
    );
}

#[cfg(feature = "json")]
#[test]
fn json_decodes_radius_values() {
    let source = r#"{"meta":{"name":"Dawn"},"radii":{"card":"8px","button":"{card}"}}"#;
    let spec: ThemeSpec = serde_json::from_str(source).expect("valid specification");

    assert_eq!(spec.radii["card"].to_string(), "8px");
    assert_eq!(spec.radii["button"].to_string(), "{card}");
}

#[cfg(feature = "json")]
#[test]
fn json_decodes_font_weight_values() {
    let source = r#"{"meta":{"name":"Dawn"},"font_weights":{"body":"400","strong":"{body}"}}"#;
    let spec: ThemeSpec = serde_json::from_str(source).expect("valid specification");

    assert_eq!(spec.font_weights["body"].to_string(), "400");
    assert_eq!(spec.font_weights["strong"].to_string(), "{body}");
}

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_color_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[colors]\n\"text.primary\" = \"#102030\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.colors["text.primary"].to_string(), "#102030");
}
