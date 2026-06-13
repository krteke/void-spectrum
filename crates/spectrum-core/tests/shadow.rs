//! Tests for the single-layer shadow contract.

use spectrum_core::{Color, Length, LengthUnit, ShadowError, ShadowLayer};

fn px(value: f32) -> Length {
    Length::new(value, LengthUnit::Px).expect("finite length")
}

#[test]
fn stores_all_shadow_components() {
    let color = Color::new_rgba(1, 2, 3, 128);
    let shadow =
        ShadowLayer::new(color, px(-1.0), px(2.0), px(4.0), px(-2.0)).expect("valid shadow");

    assert_eq!(shadow.color(), color);
    assert_eq!(shadow.offset_x(), px(-1.0));
    assert_eq!(shadow.offset_y(), px(2.0));
    assert_eq!(shadow.blur(), px(4.0));
    assert_eq!(shadow.spread(), px(-2.0));
}

#[test]
fn accepts_zero_blur() {
    assert!(ShadowLayer::new(Color::new(0, 0, 0), px(0.0), px(0.0), px(0.0), px(0.0)).is_ok());
}

#[test]
fn rejects_negative_blur() {
    assert_eq!(
        ShadowLayer::new(Color::new(0, 0, 0), px(0.0), px(0.0), px(-1.0), px(0.0)),
        Err(ShadowError::NegativeBlur)
    );
}
