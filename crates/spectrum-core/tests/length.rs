//! Tests for the platform-independent length contract.

use spectrum_core::{Length, LengthParseError, LengthUnit};

#[test]
fn parses_supported_units() {
    let cases = [
        ("12px", 12.0, LengthUnit::Px),
        ("1.25rem", 1.25, LengthUnit::Rem),
        ("-0.5em", -0.5, LengthUnit::Em),
        ("100%", 100.0, LengthUnit::Percent),
    ];

    for (input, value, unit) in cases {
        let length = input.parse::<Length>().expect("valid length");
        assert!((length.value() - value).abs() < f32::EPSILON);
        assert_eq!(length.unit(), unit);
    }
}

#[test]
fn display_round_trips() {
    let length = Length::new(1.5, LengthUnit::Rem).expect("finite length");
    assert_eq!(length.to_string().parse(), Ok(length));
}

#[test]
fn rejects_unsupported_or_missing_unit() {
    assert_eq!("12pt".parse::<Length>(), Err(LengthParseError::InvalidUnit));
    assert_eq!("12".parse::<Length>(), Err(LengthParseError::InvalidUnit));
}

#[test]
fn rejects_invalid_number() {
    assert_eq!(
        "widepx".parse::<Length>(),
        Err(LengthParseError::InvalidNumber)
    );
}

#[test]
fn rejects_non_finite_values() {
    assert_eq!(
        Length::new(f32::INFINITY, LengthUnit::Px),
        Err(LengthParseError::NonFinite)
    );
    assert_eq!("NaNpx".parse::<Length>(), Err(LengthParseError::NonFinite));
}
