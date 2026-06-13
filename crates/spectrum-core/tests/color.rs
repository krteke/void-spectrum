//! Tests for the platform-independent RGB and RGBA color contract.

use spectrum_core::{Color, ColorParseError};

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
