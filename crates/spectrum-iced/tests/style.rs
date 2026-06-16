//! Iced adapter behavior tests.

use spectrum_core::{Color, Length, LengthUnit, Radius};
use spectrum_iced::{
    IcedColorAdapter, IcedLengthAdapter, IcedLengthError, IcedRadiusAdapter, IcedRadiusError,
    color, length, radius,
};

#[test]
fn converts_rgb_colors_to_iced_rgba() {
    assert_eq!(
        Color::new(12, 34, 56).color(),
        iced_core::Color::from_rgba8(12, 34, 56, 1.0)
    );
}

#[test]
fn preserves_alpha_for_iced_colors() {
    assert_eq!(
        color(Color::new_rgba(12, 34, 56, 128)),
        iced_core::Color::from_rgba8(12, 34, 56, 128.0 / 255.0)
    );
}

#[test]
fn converts_px_lengths_to_iced_fixed_lengths() {
    let value = Length::new(24.0, LengthUnit::Px).expect("finite");

    assert_eq!(value.length(), Ok(iced_core::Length::Fixed(24.0)));
    assert_eq!(length(value), Ok(iced_core::Length::Fixed(24.0)));
}

#[test]
fn rejects_relative_lengths_without_layout_context() {
    let value = Length::new(1.5, LengthUnit::Rem).expect("finite");

    assert_eq!(
        value.length(),
        Err(IcedLengthError::UnsupportedUnit {
            unit: LengthUnit::Rem
        })
    );
}

#[test]
fn converts_px_radii_to_iced_border_radii() {
    let value = Radius::new(Length::new(6.0, LengthUnit::Px).expect("finite")).expect("radius");
    let expected = iced_core::border::Radius::new(6.0);

    assert_eq!(value.radius(), Ok(expected));
    assert_eq!(radius(value), Ok(expected));
}

#[test]
fn rejects_relative_radii_without_layout_context() {
    let value = Radius::new(Length::new(1.0, LengthUnit::Rem).expect("finite")).expect("radius");

    assert_eq!(
        value.radius(),
        Err(IcedRadiusError::UnsupportedUnit {
            unit: LengthUnit::Rem
        })
    );
}
