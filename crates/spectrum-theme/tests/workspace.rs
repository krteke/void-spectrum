//! Tests for stable facade exports.

use spectrum_theme::Color;

#[test]
fn facade_exports_stable_core_types() {
    assert_eq!(Color::new(80, 120, 200).to_string(), "#5078c8");
}
