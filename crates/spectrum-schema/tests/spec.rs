//! Tests for the top-level theme specification.

use spectrum_core::{Color, FontStyle, FontWeight, Length, LengthUnit, LineHeight, Radius};
use spectrum_schema::LineHeightValue;
use spectrum_schema::{FontStyleValue, FontWeightValue, LengthValue, RadiusValue, ThemeSpec};

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

#[test]
fn builder_adds_length_overrides() {
    let length = Length::new(8.0, LengthUnit::Px).expect("finite");
    let spec =
        ThemeSpec::new("Midnight").with_length("spacing.medium", LengthValue::Literal(length));

    assert_eq!(spec.lengths["spacing.medium"], LengthValue::Literal(length));
}

#[test]
fn builder_adds_radius_overrides() {
    let radius = Radius::new(Length::new(8.0, LengthUnit::Px).expect("finite")).expect("radius");
    let spec = ThemeSpec::new("Midnight").with_radius("card", RadiusValue::Literal(radius));

    assert_eq!(spec.radii["card"], RadiusValue::Literal(radius));
}

#[test]
fn builder_adds_font_weight_overrides() {
    let weight = FontWeight::new(600).expect("weight");
    let spec =
        ThemeSpec::new("Midnight").with_font_weight("body", FontWeightValue::Literal(weight));

    assert_eq!(spec.font_weights["body"], FontWeightValue::Literal(weight));
}

#[test]
fn builder_adds_font_style_overrides() {
    let spec = ThemeSpec::new("Midnight")
        .with_font_style("body", FontStyleValue::Literal(FontStyle::Normal));

    assert_eq!(
        spec.font_styles["body"],
        FontStyleValue::Literal(FontStyle::Normal)
    );
}

#[test]
fn builder_adds_line_height_overrides() {
    let line_height = LineHeight::multiplier(1.5).expect("line height");
    let spec =
        ThemeSpec::new("Midnight").with_line_height("body", LineHeightValue::Literal(line_height));

    assert_eq!(
        spec.line_heights["body"],
        LineHeightValue::Literal(line_height)
    );
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

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_length_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[lengths]\n\"spacing.medium\" = \"1.5rem\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.lengths["spacing.medium"].to_string(), "1.5rem");
}

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_radius_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[radii]\ncard = \"0.5rem\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.radii["card"].to_string(), "0.5rem");
}

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_font_weight_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[font_weights]\nbody = \"450\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.font_weights["body"].to_string(), "450");
}

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_font_style_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[font_styles]\nemphasis = \"oblique\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.font_styles["emphasis"].to_string(), "oblique");
}

#[cfg(feature = "toml")]
#[test]
fn toml_decodes_line_height_overrides() {
    let source = "[meta]\nname = \"Dawn\"\n\n[line_heights]\nbody = \"20px\"\n";
    let spec: ThemeSpec = toml::from_str(source).expect("valid specification");

    assert_eq!(spec.line_heights["body"].to_string(), "20px");
}
