//! Tests for the font weight contract.

use spectrum_core::{FontWeight, FontWeightParseError};

#[test]
fn accepts_open_type_range_boundaries() {
    assert_eq!(FontWeight::new(1).expect("minimum").value(), 1);
    assert_eq!(FontWeight::new(1000).expect("maximum").value(), 1000);
}

#[test]
fn supports_variable_font_weights() {
    let weight = "650".parse::<FontWeight>().expect("valid weight");
    assert_eq!(weight.value(), 650);
    assert_eq!(weight.to_string(), "650");
}

#[test]
fn exposes_common_weight_constants() {
    assert_eq!(FontWeight::NORMAL.value(), 400);
    assert_eq!(FontWeight::BOLD.value(), 700);
    assert!(FontWeight::BLACK > FontWeight::EXTRA_BOLD);
}

#[test]
fn rejects_values_outside_the_open_type_range() {
    assert_eq!(FontWeight::new(0), Err(FontWeightParseError::OutOfRange));
    assert_eq!(
        "1001".parse::<FontWeight>(),
        Err(FontWeightParseError::OutOfRange)
    );
}

#[test]
fn rejects_non_numeric_input() {
    assert_eq!(
        "bold".parse::<FontWeight>(),
        Err(FontWeightParseError::InvalidNumber)
    );
}
