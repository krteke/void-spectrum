//! Tests for the line height contract.

use spectrum_core::{Length, LengthUnit, LineHeight, LineHeightParseError};

#[test]
fn supports_multiplier_and_length_values() {
    let multiplier = "1.5".parse::<LineHeight>().expect("multiplier");
    let length = "20px".parse::<LineHeight>().expect("length");

    assert_eq!(multiplier.multiplier_value(), Some(1.5));
    assert_eq!(
        length.length_value(),
        Some(Length::new(20.0, LengthUnit::Px).expect("finite"))
    );
}

#[test]
fn display_round_trips_both_representations() {
    for input in ["1.25", "1.5rem"] {
        let value = input.parse::<LineHeight>().expect("valid line height");
        assert_eq!(value.to_string().parse(), Ok(value));
    }
}

#[test]
fn rejects_negative_values() {
    assert_eq!(
        "-1".parse::<LineHeight>(),
        Err(LineHeightParseError::Negative)
    );
    assert_eq!(
        "-2px".parse::<LineHeight>(),
        Err(LineHeightParseError::Negative)
    );
}

#[test]
fn rejects_non_finite_multiplier() {
    assert_eq!(
        "NaN".parse::<LineHeight>(),
        Err(LineHeightParseError::NonFinite)
    );
}
