//! Tests for the platform-independent RGB color contract.

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
