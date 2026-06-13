//! Tests for the corner radius contract.

use std::error::Error;

use spectrum_core::{Length, LengthParseError, LengthUnit, Radius, RadiusParseError};

#[test]
fn accepts_zero_and_positive_lengths() {
    let zero = Radius::new(Length::new(0.0, LengthUnit::Px).expect("finite"));
    let positive = "0.5rem".parse::<Radius>();

    assert!(zero.is_ok());
    assert_eq!(positive.expect("valid radius").to_string(), "0.5rem");
}

#[test]
fn exposes_the_underlying_length() {
    let length = Length::new(12.0, LengthUnit::Px).expect("finite");
    assert_eq!(Radius::new(length).expect("non-negative").length(), length);
}

#[test]
fn rejects_negative_lengths() {
    assert_eq!("-1px".parse::<Radius>(), Err(RadiusParseError::Negative));
}

#[test]
fn preserves_length_parse_errors() {
    let error = "widepx".parse::<Radius>().expect_err("invalid radius");
    assert_eq!(
        error,
        RadiusParseError::InvalidLength(LengthParseError::InvalidNumber)
    );
    assert!(error.source().is_some());
}
