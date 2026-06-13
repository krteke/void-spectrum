//! Tests for the platform-independent RGB and RGBA color contract.

use spectrum_core::{Color, ColorParseError, Rgb, Rgba};

#[test]
fn parses_uppercase_and_lowercase_hex() {
    assert_eq!(Color::from_hex("#7c3AED"), Ok(Color::new(0x7c, 0x3a, 0xed)));
}

#[test]
fn display_uses_canonical_lowercase_hex() {
    assert_eq!(Color::new(0x0a, 0xbc, 0x01).to_string(), "#0abc01");
}

#[test]
fn parses_and_displays_rgba_hex() {
    let color = Color::from_hex("#7c3AED80").expect("valid RGBA color");
    assert_eq!(color, Color::new_rgba(0x7c, 0x3a, 0xed, 0x80));
    assert_eq!(color.to_string(), "#7c3aed80");
}

#[test]
fn exposes_channels_for_both_variants() {
    let opaque = Color::new(1, 2, 3);
    let transparent = Color::new_rgba(4, 5, 6, 7);

    assert_eq!(
        (opaque.red(), opaque.green(), opaque.blue(), opaque.alpha()),
        (1, 2, 3, 255)
    );
    assert_eq!(
        (
            transparent.red(),
            transparent.green(),
            transparent.blue(),
            transparent.alpha()
        ),
        (4, 5, 6, 7)
    );
}

#[test]
fn rejects_missing_hash() {
    assert_eq!("7c3aed".parse::<Color>(), Err(ColorParseError::MissingHash));
}

#[test]
fn rejects_invalid_length() {
    assert_eq!(Color::from_hex("#fff"), Err(ColorParseError::InvalidLength));
}

#[test]
fn rejects_invalid_digit() {
    assert_eq!(
        Color::from_hex("#gg00ff"),
        Err(ColorParseError::InvalidDigit)
    );
}

#[test]
fn rgb_and_rgba_value_types_expose_channels_and_display() {
    let rgb = Rgb::new(0x12, 0x34, 0x56);
    let rgba = Rgba::new(0x12, 0x34, 0x56, 0x78);

    assert_eq!((rgb.red(), rgb.green(), rgb.blue()), (0x12, 0x34, 0x56));
    assert_eq!(
        (rgba.red(), rgba.green(), rgba.blue(), rgba.alpha()),
        (0x12, 0x34, 0x56, 0x78)
    );
    assert_eq!(rgb.to_string(), "#123456");
    assert_eq!(rgba.to_string(), "#12345678");
}

#[test]
fn rgb_and_rgba_conversions_handle_alpha_explicitly() {
    let rgb = Rgb::new(1, 2, 3);
    let rgba = Rgba::new(4, 5, 6, 7);

    assert_eq!(Rgba::from(rgb), Rgba::new(1, 2, 3, 255));
    assert_eq!(Rgb::from(rgba), Rgb::new(4, 5, 6));
    assert_eq!(rgb.into_rgba(), Rgba::new(1, 2, 3, 255));
    assert_eq!(rgba.into_rgb(), Rgb::new(4, 5, 6));
}

#[test]
fn color_normalizes_between_rgb_and_rgba() {
    let rgb = Color::from(Rgb::new(1, 2, 3));
    let rgba = Color::from(Rgba::new(4, 5, 6, 7));

    assert_eq!(rgb.rgb(), Rgb::new(1, 2, 3));
    assert_eq!(rgb.rgba(), Rgba::new(1, 2, 3, 255));
    assert_eq!(rgba.rgb(), Rgb::new(4, 5, 6));
    assert_eq!(rgba.rgba(), Rgba::new(4, 5, 6, 7));
}

#[test]
fn packed_u32_uses_rrggbbaa_byte_order() {
    let expected = Color::new_rgba(0x12, 0x34, 0x56, 0x78);

    assert_eq!(Color::from(0x1234_5678), expected);
    assert_eq!(Color::from_hex(0x1234_5678_u32), Ok(expected));
}

#[test]
fn try_from_str_matches_string_parsing() {
    assert_eq!(Color::try_from("#12345678"), "#12345678".parse::<Color>());
}
